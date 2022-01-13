use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "queryParameters")]
    pub query_parameters: QueryParameters,
}

#[derive(Serialize, Deserialize)]
pub struct QueryParameters {
    #[serde(rename = "query")]
    pub query: Option<serde_json::Value>,

    #[serde(rename = "sortColumn")]
    pub sort_column: i64,

    #[serde(rename = "sortAscending")]
    pub sort_ascending: bool,

    #[serde(rename = "maxResults")]
    pub max_results: i64,

    #[serde(rename = "page")]
    pub page: i64,

    #[serde(rename = "startDate")]
    pub start_date: Option<serde_json::Value>,

    #[serde(rename = "endDate")]
    pub end_date: Option<serde_json::Value>,

    #[serde(rename = "folderID")]
    pub folder_id: String,

    #[serde(rename = "bookmarked")]
    pub bookmarked: bool,

    #[serde(rename = "getFolderData")]
    pub get_folder_data: bool,

    #[serde(rename = "isSharedWithMe")]
    pub is_shared_with_me: bool,

    #[serde(rename = "isSubscriptionsPage")]
    pub is_subscriptions_page: bool,

    #[serde(rename = "includeArchived")]
    pub include_archived: bool,

    #[serde(rename = "includePlaylists")]
    pub include_playlists: bool,
}
