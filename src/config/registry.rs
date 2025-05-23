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

use charted_core::api::Version;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use url::Url;

/// The `[registry.<name>]` table allows to configure all the avaliable registries.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Registry {
    /// API version of the registry.
    #[serde(default)]
    pub version: Version,

    /// URL of the registry to point to. This doesn't include the API version
    /// in the URI itself (i.e, `https://charts.noelware.org/api/v1`).
    pub url: Url,
}

impl Registry {
    /// Joins the registry URL via [`Url::join`] and returns a string representation.
    pub fn join_url<T: Display>(&self, input: T) -> Result<String, url::ParseError> {
        // `format!()` is necessary here since if we tried to do 2 joins, it'll only
        // return the second join without applying the first one.
        self.url
            .join(&format!("{}/{input}", self.version))
            .map(|x| x.to_string())
    }
}

impl Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.url.join(self.version.as_str()).map_err(|_|
                /* this will map all url::ParseError as formatting errors */
                std::fmt::Error)?
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Registry;
    use charted_core::api::Version;
    use url::Url;

    #[test]
    fn url_joins() {
        let registry = Registry {
            version: Version::default(),
            url: Url::parse("https://charts.noelware.org").expect("invalid url"),
        };

        assert_eq!(
            Ok(String::from("https://charts.noelware.org/v1/weow/fluff")),
            registry.join_url("weow/fluff")
        );
    }
}
