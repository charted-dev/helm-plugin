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

use crate::{
    config::{self, Path},
    http,
};
use std::path::PathBuf;

/// Creates a Helm chart repository in a given location and creates
/// a repository on the registry itself (can be skipped with `--no-registry-creation`)
#[derive(Debug, clap::Parser)]
pub struct Args {
    location: PathBuf,
    mapping: Option<Path>,

    #[clap(flatten)]
    charted: config::Args,

    #[clap(flatten)]
    http: http::Args,
}
