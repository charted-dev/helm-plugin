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

use crate::config::{self, Config, Path};
use std::path::PathBuf;

/// Builds a local **index.yaml** of either a subset of charts or all of them.
///
/// This is similar to [**`helm repo index`**], but respects the **.charted.toml**
/// configuration file.
///
/// [**`helm repo index`**]: https://helm.sh/docs/helm/helm_repo_index/
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// A subset of charts to generate a **index.yaml** file.
    pub charts: Vec<Path>,

    /// File to generate a `index.yaml` file. Defaults to the directory where
    /// the command was executed in.
    #[arg(short = 'f', long = "file", default_value = None)]
    pub in_: Option<PathBuf>,

    #[clap(flatten)]
    charted: config::Args,
}

pub fn run(
    Args {
        charted,
        charts,
        in_: file,
    }: Args,
) -> eyre::Result<()> {
    let config = Config::load(charted.config)?;
    if charts.is_empty() {
        info!("generating `index.yaml` for all charts");
        todo!()
    }

    for chart in charts {
        if !config.repositories.contains_key(&chart) {
            error!("chart '{}' doesn't exist in .charted.toml", chart);
        }
    }

    Ok(())
}
