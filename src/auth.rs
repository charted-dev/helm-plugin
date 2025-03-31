// 🐻‍❄️🌺 charted-helm-plugin: Helm plugin to help faciliate operations with charted-server
// Copyright 2023-2025 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use base64::{Engine, engine::general_purpose};
use etcetera::{BaseStrategy, base_strategy::choose_native_strategy};
use eyre::Context as _;
use reqwest::{Url, header::HeaderValue};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

/// A multithreaded-threaded initialized string used for contextual [`Auth`] instances.
///
/// Originally, this was only a **Rc** but switched to **Arc** due to clap's
/// circumstances.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
pub struct Context(Arc<str>);

impl Context {
    /// Creates a new [`Context`] with a given string (`s`).
    pub fn new(s: impl AsRef<str>) -> Self {
        Self(Arc::from(s.as_ref()))
    }
}

impl<'s> From<&'s str> for Context {
    fn from(value: &'s str) -> Self {
        Self::new(value)
    }
}

/// A credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Credential {
    /// A fully qualified HTTP URI that points to the registry to use.
    pub registry: Url,

    /// Representation of this credential. Can be `None` if none is required.
    #[serde(
        default,
        with = "serde_yaml_ng::with::singleton_map",
        skip_serializing_if = "Option::is_none"
    )]
    pub repr: Option<Repr>,
}

/// Representation of what authentication scheme to use when requesting to
/// **charted-server**.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum Repr {
    /// Loads the API key from a system environment variable.
    EnvironmentVariable(String),

    /// Loads the authentication scheme from basic credentials.
    ///
    /// This is severely insecure! Please create an API key instead.
    Basic {
        username: String,

        #[serde(with = "crate::serde::secret_string")]
        password: SecretString,
    },

    /// Loads the authentication scheme from a constructed API key by the server
    /// that the user created.
    #[serde(with = "crate::serde::secret_string")]
    ApiKey(SecretString),
}

impl Repr {
    /// Turns the represenation of the avaliable authentication scheme into a
    /// [`HeaderValue`].
    pub fn to_header_value(&self) -> eyre::Result<HeaderValue> {
        match self {
            Repr::EnvironmentVariable(value) => {
                HeaderValue::from_str(&format!("ApiKey {value}")).context("failed to convert to header value")
            }

            Repr::Basic { username, password } => {
                let encoded = general_purpose::STANDARD.encode(format!("{username}:{}", password.expose_secret()));
                HeaderValue::from_str(&encoded).context("failed to convert to header value")
            }

            Repr::ApiKey(value) => HeaderValue::from_str(&format!("ApiKey {}", value.expose_secret()))
                .context("failed to convert to header value"),
        }
    }
}

/// Error thrown by [`Repr`]'s [`FromStr`] impl.
#[derive(Debug, derive_more::Display)]
pub enum ReprStrError {
    #[display("excessive delimiter [{delim}] received")]
    ExcessiveDelimiter { delim: char },

    #[display("invalid syntax for `basic` directive: expected `username:password`")]
    InvalidSyntax,

    #[display("unknown prefix or none given: expected `env:`, `apikey:`, or `basic:`")]
    UnknownPrefix,

    #[display("contents was empty")]
    Empty,
}

impl std::error::Error for ReprStrError {}

impl FromStr for Repr {
    type Err = ReprStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ReprStrError::Empty);
        }

        if let Some(key) = s.strip_prefix("env:") {
            return Ok(Self::EnvironmentVariable(key.to_ascii_uppercase()));
        }

        if let Some(key) = s.strip_prefix("apikey:") {
            return Ok(Self::ApiKey(SecretString::new(Box::from(key))));
        }

        if let Some(value) = s.strip_prefix("basic:") {
            if let Some((username, password)) = value.split_once(':') {
                if password.contains(':') {
                    return Err(ReprStrError::ExcessiveDelimiter { delim: ':' });
                }

                return Ok(Self::Basic {
                    username: username.to_owned(),
                    password: SecretString::new(Box::from(password)),
                });
            } else {
                return Err(ReprStrError::InvalidSyntax);
            }
        }

        Err(ReprStrError::UnknownPrefix)
    }
}

/// The schema of the `auth.yaml` in `$CONFIG_DIR/Noelware/charted-helm-plugin`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Auth {
    /// The context that is the **default** for the `helm charted context` subcommand.
    pub current: Context,

    /// A list of credentials avaliable.
    #[serde(rename = "credential")]
    pub credentials: HashMap<Context, Credential>,

    #[serde(skip)]
    opened_file_from: PathBuf,
}

impl Auth {
    /// Loads a `auth.yaml` file from a potential path (by `--auth-file` flag or the
    /// `CHARTED_HELM_AUTH_YAML` environment variable).
    #[track_caller]
    pub fn load<P: Into<Option<PathBuf>>>(potential: P) -> eyre::Result<Self> {
        let path = Auth::get_potential_file(potential)?;
        debug!(path = %path.display(), "loading `auth.yaml` file...");

        if !path.try_exists()? {
            warn!(
                "`auth.yaml` in path `{}` doesn't exist, creating new file...",
                path.display()
            );

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let default = Auth {
                current: Context::new("default"),
                credentials: azalia::hashmap!(
                    "default" => Credential {
                        repr: None,
                        registry: Url::parse("https://charts.noelware.org/api/v1").unwrap()
                    }
                ),
                opened_file_from: path.clone(),
            };

            let serialized = serde_yaml_ng::to_string(&default)?;

            let mut file = File::create_new(&path)?;

            write!(file, "{serialized}")?;
            file.flush()?;

            return Ok(default);
        }

        trace!("opening file `{}`", path.display());

        let file = File::open(&path)?;
        let mut auth = serde_yaml_ng::from_reader::<_, Auth>(file)
            .with_context(|| format!("failed to deserialize from file: {}", path.display()))?;

        auth.opened_file_from = path;
        Ok(auth)
    }

    /// Commits changes from `self` within a closure into the `auth.yaml` file.
    pub fn commit(mut self, f: impl FnOnce(&mut Self)) -> eyre::Result<()> {
        f(&mut self);
        self.save()
    }

    /// Flushes and saves the changes from `self` into the `auth.yaml` file.
    pub fn save(&self) -> eyre::Result<()> {
        debug!(path = %self.opened_file_from.display(), "saving and flushing changes to");

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.opened_file_from)?;

        let serialized = serde_yaml_ng::to_string(self)?;

        write!(file, "{serialized}")?;
        file.flush()?;

        Ok(())
    }

    fn get_potential_file<P: Into<Option<PathBuf>>>(potential: P) -> eyre::Result<PathBuf> {
        match potential.into() {
            Some(path) => Ok(path),
            None => {
                let strategy = choose_native_strategy()?;
                Ok(strategy.config_dir().join("Noelware/charted-helm-plugin/auth.yaml"))
            }
        }
    }
}

#[derive(Debug, clap::Args)]
#[group(id = "Authentication")]
pub struct Args {
    /// Location to an `auth.yaml` file that can load credentials.
    #[arg(short = 'c', long = "auth-file", env = "CHARTED_HELM_AUTH_YAML")]
    pub file: Option<PathBuf>,
}
