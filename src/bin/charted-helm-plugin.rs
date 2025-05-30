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

use charted_helm_plugin::Program;
use clap::Parser;
use std::{env, path::PathBuf};

#[cfg(not(windows))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct WorkingDirectoryGuard(PathBuf);
impl WorkingDirectoryGuard {
    fn from_current_dir() -> eyre::Result<WorkingDirectoryGuard> {
        Ok(WorkingDirectoryGuard(env::current_dir()?))
    }
}

impl Drop for WorkingDirectoryGuard {
    fn drop(&mut self) {
        tracing::trace!("changing directory to old directory {}", self.0.display());
        let _ = std::env::set_current_dir(self.0.as_path());
    }
}

fn main() -> eyre::Result<()> {
    dotenvy::dotenv().unwrap_or_default();
    color_eyre::install()?;

    let mut _guard = WorkingDirectoryGuard::from_current_dir()?;

    let program = Program::parse();
    program.init_logging();

    if let Some(dir) = program.workdir {
        std::env::set_current_dir(dir)?;
    }

    smol::block_on(program.command.run())
}
