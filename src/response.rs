use anyhow::{Context, Result};
use hyper::Body;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub categories: Vec<Category>,
    #[serde(rename = "crate")]
    pub crate_field: Crate,
    pub keywords: Vec<Keyword>,
    pub versions: Vec<Version>,
}

impl Response {
    pub async fn from_hyper(resp: hyper::Response<Body>) -> Result<Self> {
        let b = hyper::body::to_bytes(resp.into_body()).await?;
        serde_json::from_slice(b.as_ref()).context("Failed to deserialize")
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub category: String,
    #[serde(rename = "crates_cnt")]
    pub crates_cnt: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub description: String,
    pub id: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crate {
    pub badges: Vec<Value>,
    pub categories: Vec<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub description: String,
    pub documentation: Value,
    pub downloads: i64,
    #[serde(rename = "exact_match")]
    pub exact_match: bool,
    pub homepage: String,
    pub id: String,
    pub keywords: Vec<String>,
    pub links: Links,
    #[serde(rename = "max_stable_version")]
    pub max_stable_version: String,
    #[serde(rename = "max_version")]
    pub max_version: String,
    pub name: String,
    #[serde(rename = "newest_version")]
    pub newest_version: String,
    #[serde(rename = "recent_downloads")]
    pub recent_downloads: i64,
    pub repository: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub versions: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "owner_team")]
    pub owner_team: String,
    #[serde(rename = "owner_user")]
    pub owner_user: String,
    pub owners: String,
    #[serde(rename = "reverse_dependencies")]
    pub reverse_dependencies: String,
    #[serde(rename = "version_downloads")]
    pub version_downloads: String,
    pub versions: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
    #[serde(rename = "crates_cnt")]
    pub crates_cnt: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub id: String,
    pub keyword: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    #[serde(rename = "audit_actions")]
    pub audit_actions: Vec<AuditAction>,
    #[serde(rename = "crate")]
    pub crate_field: String,
    #[serde(rename = "crate_size")]
    pub crate_size: Option<i64>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dl_path")]
    pub dl_path: String,
    pub downloads: i64,
    pub features: Features,
    pub id: i64,
    pub license: Option<String>,
    pub links: Links2,
    pub num: String,
    #[serde(rename = "published_by")]
    pub published_by: Option<PublishedBy>,
    #[serde(rename = "readme_path")]
    pub readme_path: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub yanked: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditAction {
    pub action: String,
    pub time: String,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub avatar: String,
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features {
    #[serde(default)]
    pub default: Vec<String>,
    #[serde(default)]
    pub fs: Vec<String>,
    #[serde(default)]
    pub full: Vec<String>,
    #[serde(rename = "io-std")]
    #[serde(default)]
    pub io_std: Vec<String>,
    #[serde(rename = "io-util")]
    #[serde(default)]
    pub io_util: Vec<String>,
    #[serde(default)]
    pub macros: Vec<String>,
    #[serde(default)]
    pub net: Vec<String>,
    #[serde(default)]
    pub process: Vec<String>,
    #[serde(default)]
    pub rt: Vec<String>,
    #[serde(rename = "rt-multi-thread")]
    pub rt_multi_thread: Option<Vec<String>>,
    #[serde(default)]
    pub signal: Vec<String>,
    #[serde(default)]
    pub stats: Vec<Value>,
    #[serde(default)]
    pub sync: Vec<String>,
    #[serde(rename = "test-util")]
    #[serde(default)]
    pub test_util: Vec<String>,
    #[serde(default)]
    pub time: Vec<String>,
    #[serde(default)]
    pub stream: Vec<String>,
    #[serde(default)]
    pub blocking: Vec<String>,
    #[serde(default)]
    pub dns: Vec<String>,
    #[serde(rename = "io-driver")]
    pub io_driver: Option<Vec<String>>,
    #[serde(rename = "rt-core")]
    #[serde(default)]
    pub rt_core: Vec<String>,
    #[serde(rename = "rt-threaded")]
    pub rt_threaded: Option<Vec<String>>,
    #[serde(rename = "rt-util")]
    #[serde(default)]
    pub rt_util: Vec<Value>,
    #[serde(default)]
    pub tcp: Vec<String>,
    #[serde(default)]
    pub udp: Vec<String>,
    #[serde(default)]
    pub uds: Vec<String>,
    #[serde(default)]
    pub codec: Vec<String>,
    #[serde(default)]
    pub io: Vec<String>,
    #[serde(rename = "rt-current-thread")]
    pub rt_current_thread: Option<Vec<String>>,
    #[serde(rename = "rt-full")]
    #[serde(default)]
    pub rt_full: Vec<String>,
    #[serde(default)]
    pub timer: Vec<String>,
    #[serde(default)]
    pub tracing: Vec<String>,
    #[serde(default)]
    pub reactor: Vec<String>,
    #[serde(rename = "experimental-tracing")]
    #[serde(default)]
    pub experimental_tracing: Vec<String>,
    #[serde(rename = "async-await-preview")]
    #[serde(default)]
    pub async_await_preview: Vec<String>,
    #[serde(rename = "unstable-futures")]
    pub unstable_futures: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links2 {
    pub authors: String,
    pub dependencies: String,
    #[serde(rename = "version_downloads")]
    pub version_downloads: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishedBy {
    pub avatar: String,
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
    pub url: String,
}
