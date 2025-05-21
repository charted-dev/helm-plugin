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

use super::Config;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the
/// [`repository.<name>.publish`](Repository.html#structfield.publish) setting.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Publish {
    /// A simple toggle.
    ///
    /// - **true**: The repository can be published to any registry.
    /// - **false**: Repository cannot be published to any registry.
    Toggle(bool),

    /// A list of registry names that is allowed to push this repository.
    Registry(Vec<String>),
}

impl Default for Publish {
    fn default() -> Self {
        Publish::Toggle(true)
    }
}

impl Publish {
    const fn is_truthy(&self) -> bool {
        matches!(*self, Publish::Toggle(true))
    }
}

/// Register a repository that the Helm plugin can discover and allow operations on.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Repository {
    /// Source location to where the repository lives in.
    pub source: PathBuf,

    /// Indication if this repository can be published to registries.
    #[serde(default, skip_serializing_if = "Publish::is_truthy")]
    pub publish: Publish,

    /// Location to a **README** file. Defaults to
    /// <code>[`repository.<name>.source`](#structfield.source)/README.md</code>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readme: Option<PathBuf>,
}

impl Repository {
    pub fn is_publish_allowed(&self, config: &Config, registry: impl Into<String>) -> eyre::Result<bool> {
        if let Publish::Toggle(toggle) = self.publish {
            return Ok(toggle);
        }

        let allowed = match self.publish {
            Publish::Registry(ref registries) => registries.as_slice(),
            Publish::Toggle(_) => unreachable!(),
        };

        let registry = registry.into();
        if !config.registries.contains_key(&registry) {
            bail!("registry {} doesn't exist", registry)
        }

        Ok(allowed.contains(&registry))
    }
}
