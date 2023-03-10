// 🐻‍❄️📦 helm-plugin: Helm plugin for charted, to help push Helm charts into charted-server easily!~
// Copyright 2022-2023 Noelware <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

use clap::Parser;
use commands::Subcommands;
use log::LevelFilter;

pub mod api;
pub mod commands;
pub mod error;
pub mod keychain;
pub mod logging;
pub mod macros;
pub mod settings;

pub const BUILD_DATE: &str = env!("HELM_PLUGIN_BUILD_DATE");
pub const COMMIT_HASH: &str = env!("HELM_PLUGIN_COMMIT_HASH");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Parser)]
#[clap(
    about = "📦 Helm plugin made in Rust to help push Helm charts into charted-server easily!~",
    author = "Noel <cutie@floofy.dev>, Noelware Team <team@noelware.org>"
)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Subcommands,

    #[arg(
        short = 'l',
        long = "log-level",
        help = "The level for logging, regardless if `verbose` is true"
    )]
    pub log_level: Option<LevelFilter>,

    #[arg(
        short = 's',
        long = "server-url",
        help = "The instance URL to push charts towards"
    )]
    pub server_url: Option<String>,

    #[arg(short = 'v', long, help = "Enables verbose logging")]
    pub verbose: bool,
}
