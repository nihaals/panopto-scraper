use serde::Deserialize;

#[derive(Deserialize)]
pub struct Root {
    // #[serde(rename = "AllowPublicNotes")]
    // pub allow_public_notes: bool,

    // #[serde(rename = "BroadcastRefreshInterval")]
    // pub broadcast_refresh_interval: i64,

    // #[serde(rename = "BroadcastSegmentBackoff")]
    // pub broadcast_segment_backoff: i64,

    // #[serde(rename = "CompletionPercentage")]
    // pub completion_percentage: i64,
    #[serde(rename = "Delivery")]
    pub delivery: Delivery,
    // #[serde(rename = "DownloadUrl")]
    // pub download_url: Option<serde_json::Value>,

    // #[serde(rename = "EmbedUrl")]
    // pub embed_url: String,

    // #[serde(rename = "InvocationId")]
    // pub invocation_id: String,

    // #[serde(rename = "LastViewingPosition")]
    // pub last_viewing_position: f64,

    // #[serde(rename = "PodcastCompleted")]
    // pub podcast_completed: bool,

    // #[serde(rename = "SessionId")]
    // pub session_id: String,

    // #[serde(rename = "SessionRole")]
    // pub session_role: i64,

    // #[serde(rename = "UserCanCreateQuestionLists")]
    // pub user_can_create_question_lists: bool,

    // #[serde(rename = "UserEmail")]
    // pub user_email: String,

    // #[serde(rename = "UserId")]
    // pub user_id: String,

    // #[serde(rename = "UserKey")]
    // pub user_key: String,

    // #[serde(rename = "UserName")]
    // pub user_name: String,

    // #[serde(rename = "UserRating")]
    // pub user_rating: Option<serde_json::Value>,

    // #[serde(rename = "ViewerFileId")]
    // pub viewer_file_id: String,

    // #[serde(rename = "WebcastPingIntervalInSeconds")]
    // pub webcast_ping_interval_in_seconds: i64,
}

#[derive(Deserialize)]
pub struct Delivery {
    // #[serde(rename = "AllowPublishNotes")]
    // pub allow_publish_notes: bool,

    // #[serde(rename = "AudioDescriptionsEnabled")]
    // pub audio_descriptions_enabled: bool,

    // #[serde(rename = "AvailableAudioDescriptionLanguages")]
    // pub available_audio_description_languages: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "AvailableLanguages")]
    // pub available_languages: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "AverageRating")]
    // pub average_rating: i64,

    // #[serde(rename = "BroadcastEnded")]
    // pub broadcast_ended: bool,

    // #[serde(rename = "BroadcastInterrupted")]
    // pub broadcast_interrupted: bool,

    // #[serde(rename = "BroadcastType")]
    // pub broadcast_type: i64,

    // #[serde(rename = "Contributors")]
    // pub contributors: Vec<Contributor>,

    // #[serde(rename = "DisableSeekOnFirstView")]
    // pub disable_seek_on_first_view: Option<serde_json::Value>,

    // #[serde(rename = "DiscussionEnabled")]
    // pub discussion_enabled: bool,

    // #[serde(rename = "Duration")]
    // pub duration: f64,

    // #[serde(rename = "EventTargets")]
    // pub event_targets: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "FirstQuizOffset")]
    // pub first_quiz_offset: i64,

    // #[serde(rename = "HasAnyLinks")]
    // pub has_any_links: bool,

    // #[serde(rename = "HasAudioDescriptions")]
    // pub has_audio_descriptions: bool,

    // #[serde(rename = "HasCaptions")]
    // pub has_captions: bool,

    // #[serde(rename = "HasQuiz")]
    // pub has_quiz: bool,

    // #[serde(rename = "HasSplices")]
    // pub has_splices: bool,

    // #[serde(rename = "Identifier")]
    // pub identifier: String,

    // #[serde(rename = "IsActiveBroadcast")]
    // pub is_active_broadcast: bool,

    // #[serde(rename = "IsAudioPodcastEncodeComplete")]
    // pub is_audio_podcast_encode_complete: bool,

    // #[serde(rename = "IsBroadcast")]
    // pub is_broadcast: bool,

    // #[serde(rename = "IsOpen")]
    // pub is_open: bool,

    // #[serde(rename = "IsPodcastEncodeComplete")]
    // pub is_podcast_encode_complete: bool,

    // #[serde(rename = "IsPrimaryAudioOnly")]
    // pub is_primary_audio_only: bool,

    // #[serde(rename = "IsPurgedEncode")]
    // pub is_purged_encode: bool,

    // #[serde(rename = "IsPurgedLegacyEncode")]
    // pub is_purged_legacy_encode: bool,

    // #[serde(rename = "IsReadyForEditing")]
    // pub is_ready_for_editing: bool,

    // #[serde(rename = "IsStarted")]
    // pub is_started: bool,

    // #[serde(rename = "IsTabletEncodeComplete")]
    // pub is_tablet_encode_complete: bool,

    // #[serde(rename = "IsViewerEncodeAvailable")]
    // pub is_viewer_encode_available: bool,

    // #[serde(rename = "IsViewerEncodeComplete")]
    // pub is_viewer_encode_complete: bool,

    // #[serde(rename = "NextDeliveryDescription")]
    // pub next_delivery_description: Option<serde_json::Value>,

    // #[serde(rename = "NextDeliveryDuration")]
    // pub next_delivery_duration: i64,

    // #[serde(rename = "NextDeliveryId")]
    // pub next_delivery_id: Option<serde_json::Value>,

    // #[serde(rename = "NextDeliveryIsLive")]
    // pub next_delivery_is_live: bool,

    // #[serde(rename = "NextDeliveryThumbUrl")]
    // pub next_delivery_thumb_url: Option<serde_json::Value>,

    // #[serde(rename = "NextDeliveryTitle")]
    // pub next_delivery_title: Option<serde_json::Value>,

    // #[serde(rename = "NextDeliveryUrl")]
    // pub next_delivery_url: Option<serde_json::Value>,

    // #[serde(rename = "NormalizeVolume")]
    // pub normalize_volume: bool,

    // #[serde(rename = "OwnerBio")]
    // pub owner_bio: Option<serde_json::Value>,

    // #[serde(rename = "OwnerDisplayName")]
    // pub owner_display_name: String,

    // #[serde(rename = "OwnerId")]
    // pub owner_id: String,

    // #[serde(rename = "OwnerIsOverQuota")]
    // pub owner_is_over_quota: bool,

    // #[serde(rename = "Permissions")]
    // pub permissions: Vec<bool>,

    // #[serde(rename = "PodcastStreams")]
    // pub podcast_streams: Vec<Stream>,

    // #[serde(rename = "PublicID")]
    // pub public_id: String,

    // #[serde(rename = "PublicNotesStreams")]
    // pub public_notes_streams: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "RatingCount")]
    // pub rating_count: i64,

    // #[serde(rename = "RehydrationAvailable")]
    // pub rehydration_available: bool,

    // #[serde(rename = "RequiresAdvancedEditor")]
    // pub requires_advanced_editor: bool,

    // #[serde(rename = "SessionAbstract")]
    // pub session_abstract: Option<serde_json::Value>,

    // #[serde(rename = "SessionFileId")]
    // pub session_file_id: String,

    // #[serde(rename = "SessionGroupAbstract")]
    // pub session_group_abstract: Option<serde_json::Value>,

    // #[serde(rename = "SessionGroupLongName")]
    // pub session_group_long_name: String,
    #[serde(rename = "SessionGroupPublicID")]
    pub session_group_public_id: String,

    // #[serde(rename = "SessionGroupShortName")]
    // pub session_group_short_name: Option<serde_json::Value>,

    // #[serde(rename = "SessionName")]
    // pub session_name: String,

    // #[serde(rename = "SessionPublicID")]
    // pub session_public_id: String,

    // #[serde(rename = "SessionStartTime")]
    // pub session_start_time: f64,

    // #[serde(rename = "SourceDeliveryId")]
    // pub source_delivery_id: Option<serde_json::Value>,
    #[serde(rename = "Streams")]
    pub streams: Vec<Stream>,
    // #[serde(rename = "Tags")]
    // pub tags: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "Timestamps")]
    // pub timestamps: Vec<Option<serde_json::Value>>,

    // #[serde(rename = "WebcastVersionId")]
    // pub webcast_version_id: Option<serde_json::Value>,
}

// #[derive(Deserialize)]
// pub struct Contributor {
//     #[serde(rename = "Bio")]
//     pub bio: Option<serde_json::Value>,

//     #[serde(rename = "DisplayName")]
//     pub display_name: String,

//     #[serde(rename = "UserKey")]
//     pub user_key: String,
// }

#[derive(Deserialize)]
pub struct Stream {
    // #[serde(rename = "AbsoluteEnd")]
    // pub absolute_end: f64,

    // #[serde(rename = "AbsoluteStart")]
    // pub absolute_start: f64,

    // #[serde(rename = "BroadcastType")]
    // pub broadcast_type: i64,

    // #[serde(rename = "EditMediaFileType")]
    // pub edit_media_file_type: Option<i64>,

    // #[serde(rename = "EditMediaFileTypeLegacy")]
    // pub edit_media_file_type_legacy: Option<serde_json::Value>,

    // #[serde(rename = "EditMediaFileTypeName")]
    // pub edit_media_file_type_name: Option<String>,

    // #[serde(rename = "Interrupted")]
    // pub interrupted: bool,

    // #[serde(rename = "Name")]
    // pub name: Option<String>,

    // #[serde(rename = "PublicID")]
    // pub public_id: String,

    // #[serde(rename = "RelativeEnd")]
    // pub relative_end: f64,

    // #[serde(rename = "RelativeSegments")]
    // pub relative_segments: Option<serde_json::Value>,

    // #[serde(rename = "RelativeStart")]
    // pub relative_start: f64,

    // #[serde(rename = "SourceMediaFileType")]
    // pub source_media_file_type: i64,

    // #[serde(rename = "StreamFileId")]
    // pub stream_file_id: String,

    // #[serde(rename = "StreamHttpUrl")]
    // pub stream_http_url: Option<serde_json::Value>,

    // #[serde(rename = "StreamType")]
    // pub stream_type: i64,

    // #[serde(rename = "StreamTypeName")]
    // pub stream_type_name: String,
    #[serde(rename = "StreamUrl")]
    pub stream_url: String,

    // #[serde(rename = "Tag")]
    // pub tag: String,

    // #[serde(rename = "VRType")]
    // pub vr_type: i64,

    // #[serde(rename = "ViewerMediaFileType")]
    // pub viewer_media_file_type: i64,
    #[serde(rename = "ViewerMediaFileTypeName")]
    pub viewer_media_file_type_name: String,
}
