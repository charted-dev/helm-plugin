// 🐻‍❄️📦 helm-plugin: Helm plugin for charted, to help push Helm charts into charted-server easily!~
// Copyright 2022-2023 Noelware <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[allow(clippy::all)]
// This file was auto-generated by the `./tools/kt-to-rust` tool. Please do not edit this file,
// and instead run `./gradlew :tools:kt-to-rust:run` to update this file!
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepoType {
    APPLICATION,
    LIBRARY,
    OPERATOR,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiKeys {
    pub description: Option<String>,
    pub expires_in: Option<::chrono::DateTime<::chrono::Utc>>,
    pub id: u64,
    pub name: String,
    pub owner: User,
    pub scopes: u64,
    pub token: Option<String>,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizationMember {
    pub display_name: Option<String>,
    pub id: u64,
    pub joined_at: ::chrono::DateTime<::chrono::Utc>,
    pub permissions: u64,
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    pub user: User,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Organization {
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub display_name: Option<String>,
    pub flags: u64,
    pub gravatar_email: Option<String>,
    pub icon_hash: Option<String>,
    pub id: u64,
    pub name: String,
    pub twitter_handle: Option<String>,
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    pub verified_publisher: bool,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repository {
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub deprecated: bool,
    pub description: Option<String>,
    pub flags: u64,
    pub icon_hash: Option<String>,
    pub id: u64,
    pub name: String,
    pub owner_id: u64,
    #[serde(rename = "repo_type")]
    pub repo_type: RepoType,
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryMember {
    pub display_name: Option<String>,
    pub id: u64,
    pub joined_at: ::chrono::DateTime<::chrono::Utc>,
    pub permissions: u64,
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    pub user: User,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryRelease {
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub id: u64,
    pub tag: String,
    pub update_text: Option<String>,
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub avatar_hash: Option<String>,
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub description: Option<String>,
    pub flags: u64,
    pub gravatar_email: Option<String>,
    pub id: u64,
    pub name: Option<String>,
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    pub username: String,
}
