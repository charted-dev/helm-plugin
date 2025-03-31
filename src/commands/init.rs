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

/// Initializes a new chart repository structure.
///
/// **helm charted** uses a `.charted.toml` file to define a list of
/// Helm charts that can be easily published to **charted-server**
/// using the following structure:
///
/// ```toml
/// repository."myuser/repo".source = "./charts/mychart"
/// ```
///
/// This is similar to the following JSON object:
/// ```json
/// {
///     "repository": {
///         "myuser/repo": {
///             "source": "./charts/mychart"
///         }
///     }
/// }
/// ```
///
/// This will initialize a default `.charted.toml` configuration file. This will not
/// initialize any repositories as it is ambiguous to know the repository location (i.e,
/// the `myuser/repo` from above) since it needs to be known ahead-of-time.
#[derive(Debug, clap::Parser)]
pub struct Args {}
