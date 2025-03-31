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
    auth::{self, Context},
    http,
};
use charted_core::serde::Duration;
use url::Url;

/// Log into a **charted-server** registry.
///
/// The plugin uses a API key resource instead of basic authentication or
/// a session token so that it can live as long it requires to. An expiration
/// date can also be passed in via the `--expire-in` flag. If the plugin sees
/// the API key expires then it'll remove it from the `auth.yaml` file that
/// it founds the key from.
///
/// If the registry is Noelware's production server, then it'll use the login flow
/// system and will construct the API key when the flow is successful (this can be
/// disabled with `--no-login-flow`). Otherwise, it'll use the [`POST /users/login`]
/// endpoint if Basic authentication is disabled or just creates the API key with Basic
/// credentials.
///
/// [`POST /users/login`]: https://charts.noelware.org/docs/server/latest/api/reference/users#POST-/users/login
#[derive(Debug, clap::Parser)]
pub struct Args {
    registry: Url,

    /// override the name of the context for the successful login.
    name: Option<Context>,

    /// the time that this api key should expire. by default, it'll not expire.
    #[arg(long)]
    expire_in: Option<Duration>,

    /// disables the login flow for the official **charted-server** hub.
    #[arg(long)]
    no_login_flow: bool,

    /// marks the successful login as the default context.
    #[arg(long = "make-default")]
    mk_default: bool,

    #[clap(flatten)]
    http: http::Args,

    #[clap(flatten)]
    auth: auth::Args,
}
