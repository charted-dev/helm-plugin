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

use azalia::log::writers;
use charted_helm_plugin::config::Config;
use schemars::schema_for;
use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::exit,
};
use tracing::{debug, error, info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, prelude::*};

fn main() -> eyre::Result<()> {
    preinit()?;

    let arg = env::args().nth(1);
    jsonschema(arg.map(PathBuf::from))
}

fn preinit() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(azalia::log::WriteLayer::new_with(
            io::stderr(),
            writers::default::Writer {
                print_thread: false,
                print_module: false,

                ..Default::default()
            },
        ))
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .init();

    Ok(())
}

fn jsonschema(path: Option<PathBuf>) -> eyre::Result<()> {
    let path = path.unwrap_or_else(|| env::current_dir().unwrap().join("assets/charted.toml.json"));
    debug!("path: {}", path.display());

    info!("writing JSON Schema in {}", path.display());
    debug!("exists: {:?}", path.try_exists());

    let mut file = match path.try_exists() {
        Ok(true) => OpenOptions::new().write(true).open(&path)?,
        Ok(false) => {
            if let Some(parent) = path.parent() {
                if parent != Path::new("") {
                    fs::create_dir_all(parent)?;
                }
            }

            OpenOptions::new().create_new(true).write(true).open(&path)?
        }

        Err(e) => {
            error!(error = %e, path = %path.display(), "unable to validate that path exists");
            exit(1);
        }
    };

    let serialized = serde_json::to_string_pretty(&schema_for!(Config))?;
    file.write_all(serialized.as_bytes())?;
    file.flush()?;

    info!("wrote OpenAPI specification in {}", path.display());

    Ok(())
}
