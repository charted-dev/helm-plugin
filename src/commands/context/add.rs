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

use crate::auth::{self, Auth, Context, Credential, Repr};
use std::process::exit;
use url::Url;

/// Adds an authentication context.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Name of this context.
    context: Context,

    /// registry where this context belongs to
    registry: Url,

    /// the authentication itself, can be empty if no authentication is required.
    ///
    /// it can have the following values:
    ///
    /// - `env:<variable>`: uses the `$<variable>` system environment variable as the
    ///   value
    /// - `apikey:<key>`: uses a generated API key by the server.
    /// - `basic:<username>:<password>`: uses Basic authentication, do note that the
    ///   server might reject the request if it doesn't support it.
    value: Option<Repr>,

    #[clap(flatten)]
    args: auth::Args,
}

pub fn run(
    Args {
        context,
        registry,
        value,
        args: auth::Args { file },
    }: Args,
) -> eyre::Result<()> {
    let auth = Auth::load(file)?;
    if auth.credentials.contains_key(&context) {
        error!("context {} is already avaliable", context);
        info!(
            "tip: if you want to set {} as the default, use `helm charted context switch {}`",
            context, context
        );

        exit(1);
    }

    info!("adding context '{}' that points to registry {}", context, registry);
    trace!("authentication credentials: {:?}", value);

    auth.commit(|me| {
        me.credentials.insert(context, Credential { registry, repr: value });
    })
}
