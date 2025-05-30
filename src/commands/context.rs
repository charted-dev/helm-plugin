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

mod add;
mod delete;
mod export;
mod list;
mod switch;

/// Subcommands for dealing with authentication contexts.
#[derive(Debug, clap::Subcommand)]
pub enum Subcmd {
    Add(add::Args),
    Export(export::Args),
    Switch(switch::Args),
}

impl Subcmd {
    pub fn run(self) -> eyre::Result<()> {
        match self {
            Self::Add(args) => add::run(args),
            Self::Export(args) => export::run(args),
            Self::Switch(args) => switch::run(args),
        }
    }
}
