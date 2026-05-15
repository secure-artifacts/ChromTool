use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResponse {
    pub browsers: Vec<BrowserView>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordSitesResponse {
    pub browser_id: String,
    pub password_sites: Vec<PasswordSiteSummary>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserView {
    pub browser_id: String,
    pub browser_family_id: Option<String>,
    pub browser_name: String,
    pub icon_key: Option<String>,
    pub data_root: String,
    pub profiles: Vec<ProfileSummary>,
    pub extensions: Vec<ExtensionSummary>,
    pub bookmarks: Vec<BookmarkSummary>,
    pub password_sites: Vec<PasswordSiteSummary>,
    pub stats: BrowserStats,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserStats {
    pub profile_count: usize,
    pub extension_count: usize,
    pub bookmark_count: usize,
    pub password_site_count: usize,
    pub history_cleanup_profile_count: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSummary {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub emails: Vec<String>,
    pub avatar_data_url: Option<String>,
    pub avatar_icon: Option<String>,
    pub default_avatar_fill_color: Option<i64>,
    pub default_avatar_stroke_color: Option<i64>,
    pub avatar_label: String,
    pub path: String,
    pub history_cleanup: HistoryCleanupSummary,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionSummary {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub icon_data_url: Option<String>,
    pub profile_ids: Vec<String>,
    pub profiles: Vec<ExtensionAssociatedProfileSummary>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkSummary {
    pub url: String,
    pub title: String,
    pub profile_ids: Vec<String>,
    pub profiles: Vec<BookmarkAssociatedProfileSummary>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordSiteSummary {
    pub url: String,
    pub domain: String,
    pub profile_ids: Vec<String>,
    pub profiles: Vec<AssociatedProfileSummary>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoryCleanupSummary {
    pub history: CleanupFileStatus,
    pub top_sites: CleanupFileStatus,
    pub visited_links: CleanupFileStatus,
    pub shortcuts: CleanupFileStatus,
    pub sessions: CleanupFileStatus,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CleanupFileStatus {
    Found,
    Missing,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupHistoryInput {
    pub browser_id: String,
    pub profile_ids: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupHistoryResponse {
    pub results: Vec<CleanupHistoryResult>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupHistoryResult {
    pub profile_id: String,
    pub deleted_files: Vec<String>,
    pub skipped_files: Vec<String>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveExtensionsInput {
    pub browser_id: String,
    pub removals: Vec<ExtensionRemovalRequest>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBookmarksInput {
    pub browser_id: String,
    pub removals: Vec<BookmarkRemovalRequest>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionRemovalRequest {
    pub extension_id: String,
    pub profile_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkRemovalRequest {
    pub url: String,
    pub profile_ids: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveExtensionsResponse {
    pub results: Vec<RemoveExtensionResult>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBookmarksResponse {
    pub results: Vec<RemoveBookmarkResult>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveExtensionResult {
    pub extension_id: String,
    pub profile_id: String,
    pub removed_files: Vec<String>,
    pub skipped_files: Vec<String>,
    pub error: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBookmarkResult {
    pub url: String,
    pub profile_id: String,
    pub removed_count: usize,
    pub removed_files: Vec<String>,
    pub skipped_files: Vec<String>,
    pub error: Option<String>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedProfileSummary {
    pub id: String,
    pub name: String,
    pub avatar_data_url: Option<String>,
    pub avatar_icon: Option<String>,
    pub default_avatar_fill_color: Option<i64>,
    pub default_avatar_stroke_color: Option<i64>,
    pub avatar_label: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionAssociatedProfileSummary {
    pub id: String,
    pub name: String,
    pub avatar_data_url: Option<String>,
    pub avatar_icon: Option<String>,
    pub default_avatar_fill_color: Option<i64>,
    pub default_avatar_stroke_color: Option<i64>,
    pub avatar_label: String,
    pub install_source: ExtensionInstallSourceSummary,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionInstallSourceSummary {
    Store,
    External,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkAssociatedProfileSummary {
    pub id: String,
    pub name: String,
    pub avatar_data_url: Option<String>,
    pub avatar_icon: Option<String>,
    pub default_avatar_fill_color: Option<i64>,
    pub default_avatar_stroke_color: Option<i64>,
    pub avatar_label: String,
    pub bookmark_path: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserConfigListResponse {
    pub configs: Vec<BrowserConfigEntry>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowserConfigEntry {
    pub id: String,
    pub source: BrowserConfigSource,
    pub browser_family_id: Option<String>,
    pub icon_key: Option<String>,
    pub name: String,
    pub executable_path: String,
    pub user_data_path: String,
    pub deletable: bool,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BrowserConfigSource {
    Default,
    Custom,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomBrowserConfigInput {
    pub name: String,
    pub icon_key: Option<String>,
    pub executable_path: String,
    pub user_data_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoredBrowserConfigs {
    pub custom_configs: Vec<CustomBrowserConfigRecord>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomBrowserConfigRecord {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub icon_key: Option<String>,
    #[serde(default)]
    pub browser_family_id: Option<String>,
    pub executable_path: String,
    pub user_data_path: String,
}

pub struct BrowserDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub local_app_data_segments: &'static [&'static str],
    pub executable_candidates: &'static [crate::browsers::ExecutableCandidate],
}

pub struct TempExtension {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub icon_data_url: Option<String>,
    pub profile_ids: BTreeSet<String>,
    pub profiles: BTreeMap<String, ExtensionAssociatedProfileSummary>,
}

pub struct TempBookmark {
    pub url: String,
    pub title: String,
    pub profile_ids: BTreeSet<String>,
    pub profiles: BTreeMap<String, BookmarkAssociatedProfileSummary>,
}

pub struct TempPasswordSite {
    pub url: String,
    pub domain: String,
    pub profile_ids: BTreeSet<String>,
    pub profiles: BTreeMap<String, AssociatedProfileSummary>,
}
