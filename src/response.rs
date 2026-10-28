use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

const BASE: &str = "https://crates.io/crates";

/// Crates.io search result containing crate metadata and related records.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Categories associated with the result.
    pub categories: Vec<Category>,
    /// Metadata for the matching crate.
    #[serde(rename = "crate")]
    pub crate_field: Crate,
    /// Keywords associated with the result.
    pub keywords: Vec<Keyword>,
    /// Published versions of the crate.
    pub versions: Vec<Version>,
}

impl Response {
    /// Returns the crate homepage, if one is set.
    pub fn homepage(&self) -> Option<String> {
        self.crate_field.homepage.clone()
    }

    /// Returns the configured documentation URL or the docs.rs page.
    pub fn documentation(&self) -> String {
        if let Some(s) = self.crate_field.documentation.as_str() {
            return s.to_string();
        }
        format!("https://docs.rs/{}", self.crate_field.name)
    }

    /// Returns the crates.io page URL for this crate.
    pub fn crates(&self) -> String {
        format!("{BASE}/{}", self.crate_field.name)
    }
}

/// Category metadata returned by crates.io.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    /// Category display name.
    pub category: String,
    /// Number of crates in this category.
    #[serde(rename = "crates_cnt")]
    pub crates_cnt: i64,
    /// Category creation timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Category description.
    pub description: String,
    /// Category identifier.
    pub id: String,
    /// URL-friendly category name.
    pub slug: String,
}

/// Metadata for a crate on crates.io.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crate {
    /// Badge metadata.
    pub badges: Vec<Value>,
    /// Category identifiers.
    pub categories: Vec<String>,
    /// Crate creation timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Crate description.
    pub description: String,
    /// Documentation URL, when available.
    pub documentation: Value,
    /// Total download count.
    pub downloads: i64,
    /// Whether this result exactly matches the query.
    #[serde(rename = "exact_match")]
    pub exact_match: bool,
    /// Crate homepage URL, when available.
    pub homepage: Option<String>,
    /// Crate identifier.
    pub id: String,
    /// Crate keywords.
    pub keywords: Vec<String>,
    /// Related crates.io API links.
    pub links: Links,
    /// Latest stable version.
    #[serde(rename = "max_stable_version")]
    pub max_stable_version: String,
    /// Latest published version.
    #[serde(rename = "max_version")]
    pub max_version: String,
    /// Crate name.
    pub name: String,
    /// Most recently published version.
    #[serde(rename = "newest_version")]
    pub newest_version: String,
    /// Downloads in the recent reporting period.
    #[serde(rename = "recent_downloads")]
    pub recent_downloads: i64,
    /// Source repository URL.
    pub repository: String,
    /// Last update timestamp.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Identifiers of published versions.
    pub versions: Vec<i64>,
}

/// API links associated with a crate.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    /// URL for the crate's owner team.
    #[serde(rename = "owner_team")]
    pub owner_team: String,
    /// URL for the crate's owner user.
    #[serde(rename = "owner_user")]
    pub owner_user: String,
    /// URL for the crate's owners.
    pub owners: String,
    /// URL for reverse dependencies.
    #[serde(rename = "reverse_dependencies")]
    pub reverse_dependencies: String,
    /// URL for version download statistics.
    #[serde(rename = "version_downloads")]
    pub version_downloads: String,
    /// URL or data for crate versions.
    pub versions: Value,
}

/// Keyword metadata returned by crates.io.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
    /// Number of crates using this keyword.
    #[serde(rename = "crates_cnt")]
    pub crates_cnt: i64,
    /// Keyword creation timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Keyword identifier.
    pub id: String,
    /// Keyword text.
    pub keyword: String,
}

/// Metadata for a published crate version.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    /// Audit actions for this version.
    #[serde(rename = "audit_actions")]
    pub audit_actions: Vec<AuditAction>,
    /// Name of the crate containing this version.
    #[serde(rename = "crate")]
    pub crate_field: String,
    /// Downloaded archive size, when available.
    #[serde(rename = "crate_size")]
    pub crate_size: Option<i64>,
    /// Version publication timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Download path for the version archive.
    #[serde(rename = "dl_path")]
    pub dl_path: String,
    /// Download count for this version.
    pub downloads: i64,
    /// Feature definitions.
    pub features: Value,
    /// Version identifier.
    pub id: i64,
    /// License expression, when available.
    pub license: Option<String>,
    /// API links associated with this version.
    pub links: Links2,
    /// Version number.
    pub num: String,
    /// Publisher metadata, when available.
    #[serde(rename = "published_by")]
    pub published_by: Option<PublishedBy>,
    /// Readme path for this version.
    #[serde(rename = "readme_path")]
    pub readme_path: String,
    /// Last update timestamp.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Whether the version has been yanked.
    pub yanked: bool,
}

/// Audit action recorded for a crate version.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditAction {
    /// Action performed.
    pub action: String,
    /// Time the action was recorded.
    pub time: String,
    /// User who performed the action.
    pub user: User,
}

/// User metadata returned by crates.io.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User avatar URL.
    pub avatar: String,
    /// User identifier.
    pub id: i64,
    /// User login name.
    pub login: String,
    /// Display name, when available.
    pub name: Option<String>,
    /// User profile URL.
    pub url: String,
}

/// API links associated with a version.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links2 {
    /// URL for version authors.
    pub authors: String,
    /// URL for version dependencies.
    pub dependencies: String,
    /// URL for version download statistics.
    #[serde(rename = "version_downloads")]
    pub version_downloads: String,
}

/// Publisher metadata for a crate version.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishedBy {
    /// Publisher avatar URL.
    pub avatar: String,
    /// Publisher identifier.
    pub id: i64,
    /// Publisher login name.
    pub login: String,
    /// Publisher display name, when available.
    pub name: Option<String>,
    /// Publisher profile URL.
    pub url: String,
}
