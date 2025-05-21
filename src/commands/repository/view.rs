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

use crate::config::{self, Config, Path, repository::Repository};
use charted_helm_types::Chart;
use eyre::Context;
use serde_json::json;
use std::{fs::File, process::exit};

/// Views a repository's metadata from the `.charted.toml` file.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// a `owner/repo` mapping to view a single repository's metadata
    repo: Option<Path>,

    /// reports as JSON instead of plain text
    #[arg(short = 'j', long, default_value_t = false)]
    json: bool,

    #[clap(flatten)]
    charted: config::Args,
}

pub fn run(Args { repo, charted, json }: Args) -> eyre::Result<()> {
    let config = Config::load(charted.config)?;
    let Some(repo) = repo else {
        for (path, repo) in &config.repositories {
            print(path, repo, json)?;
            if !json {
                println!("---");
                println!();
            }
        }

        return Ok(());
    };

    let Some(repository) = config.repositories.get(&repo) else {
        error!("chart '{}' doesn't exist", repo);
        exit(1);
    };

    print(&repo, repository, json)
}

fn print(path: &Path, repo: &Repository, json: bool) -> eyre::Result<()> {
    let src = repo
        .source
        .canonicalize()
        .with_context(|| format!("failed to canonicalize path: {}", repo.source.display()))?;

    debug!(src = %src.display(), "resolving chart spec from source directory");

    let file = File::open(src.join("Chart.yaml"))?;
    let chart: Chart = serde_yaml_ng::from_reader(file)?;
    if json {
        let data = json!({
            "publish": repo.publish,
            "source": src,
            "spec": chart,
        });

        println!("{}", serde_json::to_string_pretty(&data).unwrap());
        return Ok(());
    }

    println!("Helm Chart {} v{} ({})", chart.name, chart.version, path);
    println!("~> Source: {}", src.display());

    Ok(())
}
