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

use charted_core::serde::Duration;
use std::{ffi::OsStr, fs::File, io::Read, path::PathBuf, str::FromStr};

/// a certificate that the HTTP client will load.
///
/// this can be used if the server uses a custom certificate or a self-signed one.
#[derive(Debug, Clone, derive_more::Display)]
#[display("certificate '{}' in {kind} format", self.path.display())]
pub struct Certificate {
    path: PathBuf,
    kind: CertKind,
}

impl TryInto<reqwest::Certificate> for Certificate {
    type Error = reqwest::Error;

    fn try_into(self) -> Result<reqwest::Certificate, Self::Error> {
        match self.kind {
            CertKind::Pem => {
                let mut buf = Vec::new();
                File::open(self.path).unwrap().read_to_end(&mut buf).unwrap();

                Ok(reqwest::Certificate::from_pem(&buf)?)
            }

            CertKind::Der => {
                let mut buf = Vec::new();
                File::open(self.path).unwrap().read_to_end(&mut buf).unwrap();

                Ok(reqwest::Certificate::from_der(&buf)?)
            }
        }
    }
}

#[derive(Debug, derive_more::Display)]
pub enum CertificateFromStrError {
    #[display("unable to infer certificate representation from path '{}', expected either `.pem` or `.der` as the path extension", path.display())]
    UnknownExtension { path: PathBuf },

    #[display("invalid extension '.{ext}' in path {}", path.display())]
    InvalidExtension { path: PathBuf, ext: String },

    #[display("extension in path is not valid unicode")]
    NotUnicode,

    #[display("empty input")]
    Empty,
}

impl std::error::Error for CertificateFromStrError {}

impl FromStr for Certificate {
    type Err = CertificateFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(CertificateFromStrError::Empty);
        }

        if let Some(path) = s.strip_prefix("pem:").map(PathBuf::from) {
            return Ok(Certificate {
                path,
                kind: CertKind::Pem,
            });
        }

        if let Some(path) = s.strip_prefix("der:").map(PathBuf::from) {
            return Ok(Certificate {
                path,
                kind: CertKind::Der,
            });
        }

        let path = PathBuf::from(s);
        Ok(Certificate {
            kind: match path.extension().map(OsStr::to_str) {
                Some(Some("pem")) => CertKind::Pem,
                Some(Some("der")) => CertKind::Der,
                Some(Some(s)) => {
                    return Err(CertificateFromStrError::InvalidExtension {
                        path: path.clone(),
                        ext: s.to_owned(),
                    });
                }

                Some(None) => return Err(CertificateFromStrError::NotUnicode),
                None => Default::default(),
            },

            path,
        })
    }
}

/// certificate format
#[derive(Debug, Clone, Copy, Default, derive_more::Display)]
pub enum CertKind {
    #[display("pem")]
    #[default]
    Pem,

    #[display("der")]
    Der,
}

#[derive(Debug, clap::Args)]
#[group(id = "HTTP")]
pub struct Args {
    #[arg(long, default_value_t = __default_connect_timeout(), env = "CHARTED_HELM_HTTP_CONNECT_TIMEOUT")]
    pub connect_timeout: Duration,

    #[arg(long = "certificate", env = "CHARTED_HELM_HTTP_CERTS")]
    pub certificates: Vec<Certificate>,
}

const fn __default_connect_timeout() -> Duration {
    Duration::from_secs(5)
}
