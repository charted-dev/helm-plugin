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

use std::path::PathBuf;
use url::Url;

/// Implements Helm's [Downloader Protocol] feature for plugins.
///
/// [Downloader Protocol]: https://helm.sh/docs/topics/plugins/#downloader-plugins
#[derive(Debug, clap::Parser)]
pub struct Args {
    cert_file: PathBuf,
    key_file: PathBuf,
    ca_file: PathBuf,
    url: Url,
}

pub async fn run(
    Args {
        cert_file,
        key_file,
        ca_file,
        url,
    }: Args,
) -> eyre::Result<()> {
    info!(
        "cert_file={}; key_file={}; ca_file={}; url={url}",
        cert_file.display(),
        key_file.display(),
        ca_file.display()
    );

    Ok(())
}
