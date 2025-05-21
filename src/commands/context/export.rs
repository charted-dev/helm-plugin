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

use crate::auth::{self, Auth, Context};
use serde_json::json;
use std::process::exit;

/// Exports a context as JSON.
#[derive(Debug, clap::Parser)]
pub struct Args {
    context: Option<Context>,

    #[clap(flatten)]
    auth: auth::Args,
}

pub fn run(Args { context, auth }: Args) -> eyre::Result<()> {
    let auth = Auth::load(auth.file)?;
    let Some(context) = context else {
        // paranoia check
        assert!(auth.credentials.contains_key(&auth.current));

        let credential = unsafe { auth.credentials.get(&auth.current).unwrap_unchecked() };
        println!(
            "{}",
            serde_json::to_string(&json!({
                "context": auth.current,
                "credential": credential
            }))?
        );

        return Ok(());
    };

    if !auth.credentials.contains_key(&context) {
        warn!("context '{}' doesn't exist!", context);
        exit(1);
    }

    let credential = unsafe { auth.credentials.get(&context).unwrap_unchecked() };
    println!(
        "{}",
        serde_json::to_string(&json!({
            "context": context,
            "credential": credential
        }))?
    );

    Ok(())
}
