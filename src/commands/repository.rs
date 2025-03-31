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

mod create;
mod updatedeps;
mod view;

/// Subcommands for dealing with operations related to Helm chart repositories.
#[derive(Debug, clap::Subcommand)]
#[clap(alias = "repo")]
pub enum Subcmd {
    Create(create::Args),

    #[command(name = "update-deps", alias("depupdate"), alias("updatedeps"))]
    UpdateDependencies(updatedeps::Args),
    View(view::Args),
}

impl Subcmd {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Self::View(_) => todo!(),
            Self::UpdateDependencies(_) => todo!(),
            Self::Create(_) => todo!(),
        }
    }
}
