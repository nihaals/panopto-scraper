use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "d")]
    pub d: D,
}

#[derive(Serialize, Deserialize)]
pub struct D {
    #[serde(rename = "__type")]
    pub d_type: String,

    #[serde(rename = "Results")]
    pub results: Vec<Result>,

    #[serde(rename = "TotalNumber")]
    pub total_number: i64,

    #[serde(rename = "BookmarkedSessionCount")]
    pub bookmarked_session_count: Option<serde_json::Value>,

    #[serde(rename = "CanAddSubfolder")]
    pub can_add_subfolder: bool,

    #[serde(rename = "ContainsArchivedDeliveries")]
    pub contains_archived_deliveries: bool,

    #[serde(rename = "FolderType")]
    pub folder_type: i64,

    #[serde(rename = "InProgressSessionCount")]
    pub in_progress_session_count: Option<serde_json::Value>,

    #[serde(rename = "ParentFolderId")]
    pub parent_folder_id: Option<serde_json::Value>,

    #[serde(rename = "ScheduledRecordingCount")]
    pub scheduled_recording_count: Option<serde_json::Value>,

    #[serde(rename = "Subfolders")]
    pub subfolders: Vec<Subfolder>,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "__type")]
    pub result_type: ResultType,

    #[serde(rename = "Abstract")]
    pub result_abstract: String,

    #[serde(rename = "AffiliationName")]
    pub affiliation_name: Option<serde_json::Value>,

    #[serde(rename = "AnalyticsAllTimeViewCount")]
    pub analytics_all_time_view_count: i64,

    #[serde(rename = "AnalyticsCompletionPercentage")]
    pub analytics_completion_percentage: Option<serde_json::Value>,

    #[serde(rename = "ArchiveEstimatedRestoreTime")]
    pub archive_estimated_restore_time: Option<serde_json::Value>,

    #[serde(rename = "ArchivedByUsername")]
    pub archived_by_username: Option<serde_json::Value>,

    #[serde(rename = "ArchivedTime")]
    pub archived_time: Option<serde_json::Value>,

    #[serde(rename = "AvailabilityMessage")]
    pub availability_message: String,

    #[serde(rename = "AvailabilityWindowEnd")]
    pub availability_window_end: Option<serde_json::Value>,

    #[serde(rename = "AvailabilityWindowStart")]
    pub availability_window_start: Option<serde_json::Value>,

    #[serde(rename = "AverageRating")]
    pub average_rating: i64,

    #[serde(rename = "Bookmarks")]
    pub bookmarks: Vec<Option<serde_json::Value>>,

    #[serde(rename = "Context")]
    pub context: Vec<Option<serde_json::Value>>,

    #[serde(rename = "ContextsHaveTime")]
    pub contexts_have_time: bool,

    #[serde(rename = "DatabaseSessionStatus")]
    pub database_session_status: i64,

    #[serde(rename = "DeletedByUsername")]
    pub deleted_by_username: Option<serde_json::Value>,

    #[serde(rename = "DeletedTime")]
    pub deleted_time: String,

    #[serde(rename = "DeliveryID")]
    pub delivery_id: String,

    #[serde(rename = "DeliveryName")]
    pub delivery_name: DeliveryName,

    #[serde(rename = "Duration")]
    pub duration: f64,

    #[serde(rename = "EmbedUrl")]
    pub embed_url: String,

    #[serde(rename = "FolderID")]
    pub folder_id: String,

    #[serde(rename = "FolderName")]
    pub folder_name: String,

    #[serde(rename = "HasCaptions")]
    pub has_captions: bool,

    #[serde(rename = "HasStreams")]
    pub has_streams: bool,

    #[serde(rename = "HasWriteAccess")]
    pub has_write_access: bool,

    #[serde(rename = "IosVideoUrl")]
    pub ios_video_url: String,

    #[serde(rename = "IsArchived")]
    pub is_archived: bool,

    #[serde(rename = "IsBroadcast")]
    pub is_broadcast: bool,

    #[serde(rename = "IsCreator")]
    pub is_creator: bool,

    #[serde(rename = "IsDownloadAvailable")]
    pub is_download_available: Option<serde_json::Value>,

    #[serde(rename = "IsEditable")]
    pub is_editable: bool,

    #[serde(rename = "IsPermanentlyDeletable")]
    pub is_permanently_deletable: bool,

    #[serde(rename = "IsRestorable")]
    pub is_restorable: bool,

    #[serde(rename = "MostRecentViewPosition")]
    pub most_recent_view_position: f64,

    #[serde(rename = "MostRecentViewing")]
    pub most_recent_viewing: Option<serde_json::Value>,

    #[serde(rename = "NotesUrl")]
    pub notes_url: String,

    #[serde(rename = "OwnerFullName")]
    pub owner_full_name: Option<serde_json::Value>,

    #[serde(rename = "OwnerUserId")]
    pub owner_user_id: Option<serde_json::Value>,

    #[serde(rename = "PendingPermanentDeletionTime")]
    pub pending_permanent_deletion_time: Option<serde_json::Value>,

    #[serde(rename = "Permissions")]
    pub permissions: Vec<bool>,

    #[serde(rename = "PlayableObjectType")]
    pub playable_object_type: i64,

    #[serde(rename = "PodcastDetails")]
    pub podcast_details: i64,

    #[serde(rename = "PresenterFirstNames")]
    pub presenter_first_names: Vec<Option<serde_json::Value>>,

    #[serde(rename = "PresenterLastNames")]
    pub presenter_last_names: Vec<Option<serde_json::Value>>,

    #[serde(rename = "PresenterUserNames")]
    pub presenter_user_names: Vec<Option<serde_json::Value>>,

    #[serde(rename = "Progress")]
    pub progress: i64,

    #[serde(rename = "RemoteRecorderID")]
    pub remote_recorder_id: Option<serde_json::Value>,

    #[serde(rename = "RemoteRecorderName")]
    pub remote_recorder_name: Option<serde_json::Value>,

    #[serde(rename = "RetentionActionType")]
    pub retention_action_type: Option<serde_json::Value>,

    #[serde(rename = "RetentionApprovalUserId")]
    pub retention_approval_user_id: Option<serde_json::Value>,

    #[serde(rename = "RetentionApprovalUserName")]
    pub retention_approval_user_name: Option<serde_json::Value>,

    #[serde(rename = "RetentionRuleGroupName")]
    pub retention_rule_group_name: Option<serde_json::Value>,

    #[serde(rename = "RetentionRuleName")]
    pub retention_rule_name: Option<serde_json::Value>,

    #[serde(rename = "RetentionRuleViolationDate")]
    pub retention_rule_violation_date: Option<serde_json::Value>,

    #[serde(rename = "SessionGroupRequiresApproval")]
    pub session_group_requires_approval: bool,

    #[serde(rename = "SessionID")]
    pub session_id: String,

    #[serde(rename = "SessionName")]
    pub session_name: String,

    #[serde(rename = "SharedWithMeDate")]
    pub shared_with_me_date: Option<serde_json::Value>,

    #[serde(rename = "ShowSettings")]
    pub show_settings: bool,

    #[serde(rename = "StartTime")]
    pub start_time: String,

    #[serde(rename = "Status")]
    pub status: i64,

    #[serde(rename = "TabletVideoUrl")]
    pub tablet_video_url: String,

    #[serde(rename = "Tags")]
    pub tags: Vec<Option<serde_json::Value>>,

    #[serde(rename = "ThumbUrl")]
    pub thumb_url: String,

    #[serde(rename = "UserCanEnumerateFolder")]
    pub user_can_enumerate_folder: bool,

    #[serde(rename = "ViewerUrl")]
    pub viewer_url: String,

    #[serde(rename = "EditorUrl")]
    pub editor_url: String,

    #[serde(rename = "OrderSort")]
    pub order_sort: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Subfolder {
    #[serde(rename = "__type")]
    pub subfolder_type: SubfolderType,

    #[serde(rename = "Abstract")]
    pub subfolder_abstract: Option<serde_json::Value>,

    #[serde(rename = "AffiliationName")]
    pub affiliation_name: Option<serde_json::Value>,

    #[serde(rename = "CanDownload")]
    pub can_download: bool,

    #[serde(rename = "Context")]
    pub context: Option<serde_json::Value>,

    #[serde(rename = "DeliveriesSpecifiedOrder")]
    pub deliveries_specified_order: bool,

    #[serde(rename = "FolderType")]
    pub folder_type: i64,

    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "IsCreator")]
    pub is_creator: bool,

    #[serde(rename = "IsDropbox")]
    pub is_dropbox: bool,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "ParentFolderId")]
    pub parent_folder_id: Option<serde_json::Value>,

    #[serde(rename = "ParentFolderName")]
    pub parent_folder_name: Option<serde_json::Value>,

    #[serde(rename = "PodcastEnabled")]
    pub podcast_enabled: bool,

    #[serde(rename = "Presenters")]
    pub presenters: Option<serde_json::Value>,

    #[serde(rename = "SessionCount")]
    pub session_count: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub enum DeliveryName {
    #[serde(rename = "default")]
    Default,
}

#[derive(Serialize, Deserialize)]
pub enum ResultType {
    #[serde(rename = "SessionRow:#Panopto.Data.DB.WebUI")]
    SessionRowPanoptoDataDbWebUi,
}

#[derive(Serialize, Deserialize)]
pub enum SubfolderType {
    #[serde(rename = "FolderRow:#Panopto.Data.DB.WebUI")]
    FolderRowPanoptoDataDbWebUi,
}
