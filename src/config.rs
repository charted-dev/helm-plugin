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

use charted_core::api;
use charted_types::name::{self, Name};
use eyre::Context;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    str::FromStr,
};
use url::Url;

pub mod global;
pub mod registry;
pub mod repository;

/// A repository path that is joined from the first [`Name`], which is the
/// owner of the repository and the secondary [`Name`], which is the repository
/// name.
///
/// ## Examples
/// ```plaintext
/// noel/ume
/// uwuDaOwO~/name
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
#[display("{}/{}", self.owner, self.repository)]
pub struct Path {
    pub owner: Name,
    pub repository: Name,
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}/{}", self.owner, self.repository))
    }
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = Path;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("valid mapping of {{owner}}/{{repo}}")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// Error variant for the [`FromStr`] impl for [`Path`].
#[derive(Debug, derive_more::Display)]
pub enum PathFromStrError {
    #[display("excessive delimiter '{delim}' reached, only a single `{delim}` should be present.")]
    ExcessiveDelimiter {
        delim: char,
    },

    #[display("invalid syntax, must be in 'owner/repo' form")]
    InvalidSyntax,

    #[display("contents was empty")]
    Empty,

    Name(name::Error),
}

impl std::error::Error for PathFromStrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Name(name) => Some(name),
            _ => None,
        }
    }
}

impl FromStr for Path {
    type Err = PathFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(PathFromStrError::Empty);
        }

        match s.split_once('/') {
            Some((_, repo)) if repo.contains('/') => Err(PathFromStrError::ExcessiveDelimiter { delim: '/' }),
            Some((owner, repo)) => Ok(Path {
                owner: owner.parse().map_err(PathFromStrError::Name)?,
                repository: repo.parse().map_err(PathFromStrError::Name)?,
            }),

            None => Err(PathFromStrError::InvalidSyntax),
        }
    }
}

/// Configuration file for configuring repositories when authoring charts
/// that are pushed into [charted-server].
///
/// ## Example
/// ```toml
/// [global]
/// # semver constraint of what version `charted-helm-plugin` to require.
/// plugin = ">= 0.1"
///
/// # semver constraint of what version of `helm` to require.
/// helm   = ">= 3.12"
///
/// [repository."noelware/my-chart"]
/// source = "./charts/my-chart"
/// ```
///
/// To view the properties of **my-chart**, you can use the **repository view**
/// subcommand:
///
/// ```shell
/// $ helm charted repository view my-chart
/// Chart `my-chart`:
///     -> Registry:                    default (https://charts.noelware.org/api/v1)
///     -> Source:                      /git/Noelware/helm-charts/charts/my-chart
///     -> Version (from `Chart.yaml`): 0.1.0
/// ```
///
/// [charted-server]: https://charts.noelware.org
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Config {
    /// Global configuration that affects the lifecycle of the plugin.
    #[serde(default)]
    pub global: global::Global,

    /// A set of registries that are avaliable to each repository.
    #[serde(default, rename = "registry", skip_serializing_if = "BTreeMap::is_empty")]
    pub registries: BTreeMap<String, registry::Registry>,

    /// A set of repositories determined by a **path**, which is `{{owner}}/{{repo}}`.
    #[serde(default, rename = "repository", skip_serializing_if = "BTreeMap::is_empty")]
    pub repositories: BTreeMap<Path, repository::Repository>,

    // allows keeping track of what file we are for `flush_and_save`.
    #[serde(skip)]
    #[schemars(skip)]
    opened_from: PathBuf,
}

impl Config {
    pub fn load<P: Into<Option<PathBuf>>>(potential: P) -> eyre::Result<Self> {
        let path = Config::get_potential_default_path(potential)?;
        debug!(path = %path.display(), "loading plugin configuration in");

        if !path.try_exists()? {
            bail!("configuration file in location '{}' doesn't exist", path.display())
        }

        trace!("opening file `{}`", path.display());

        let mut config = toml::from_str::<Self>(&fs::read_to_string(&path)?)
            .with_context(|| format!("failed to deserialize from file: {}", path.display()))?;

        if !config.registries.contains_key("default") {
            config.registries.insert(String::from("default"), registry::Registry {
                version: api::Version::V1,
                url: Url::parse("https://charts.noelware.org/api").unwrap(),
            });
        }

        config.opened_from = path;
        Ok(config)
    }

    pub fn flush_and_save(&self) -> eyre::Result<()> {
        debug!(path = %self.opened_from.display(), "saving and flushing changes");

        let mut file = OpenOptions::new().write(true).open(&self.opened_from)?;
        let serialized = toml::to_string_pretty(self)?;

        write!(file, "{serialized}")?;
        file.flush()?;

        Ok(())
    }

    fn get_potential_default_path<P: Into<Option<PathBuf>>>(potential: P) -> eyre::Result<PathBuf> {
        if let Some(path) = potential.into() {
            return Ok(path);
        }

        for p in [std::path::Path::new("./.charted.toml"), std::path::Path::new("./charted.toml")] {
            trace!(potential = %p.display(), "checking if path exists");

            if p.try_exists()? {
                trace!("using potential configuration file: {}", p.display());
                return Ok(p.to_path_buf());
            }
        }

        bail!(
            "No potential `charted.toml` files found in default locations (.charted.toml, charted.toml in current directory). Initialize with `helm charted init`."
        )
    }
}

#[derive(Debug, clap::Args)]
#[group(id = "Configuration")]
pub struct Args {
    /// Location to an `.charted.toml` file.
    #[arg(short = 'c', long, env = "CHARTED_HELM_TOML_FILE")]
    pub config: Option<PathBuf>,
}
