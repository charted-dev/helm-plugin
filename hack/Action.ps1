#!/usr/bin/env pwsh
#
# 🐻‍❄️🌺 charted-helm-plugin: Helm plugin to help faciliate operations with charted-server
# Copyright 2023-2025 Noelware, LLC. <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

[CmdletBinding()]
Param(
    # which version of `charted-helm-plugin` to install or update
    [String]$Version = "latest",

    # if `-NoCompletions` is set, it wont install shell completions
    # for 'helm charted'
    [Switch]$NoCompletions = $false,

    # `-Update` switch to update the binary rather than install a new one.
    [Switch]$Update = $false
)

function Get-BinaryUri {
    Param([String]$Version)

    return "https://artifacts.noelware.org/charted/helm-plugin/$Version/charted-helm-plugin-windows-x86_64.exe"
}

function Get-ChecksumUri {
    Param([String]$Version)

    return "https://artifacts.noelware.org/charted/helm-plugin/$Version/charted-helm-plugin-windows-x86_64.exe.sha256"
}
