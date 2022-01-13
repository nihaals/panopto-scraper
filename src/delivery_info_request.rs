use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "deliveryId")]
    pub delivery_id: String,

    #[serde(rename = "invocationId")]
    pub invocation_id: String,

    #[serde(rename = "isLiveNotes")]
    pub is_live_notes: bool,

    #[serde(rename = "refreshAuthCookie")]
    pub refresh_auth_cookie: bool,

    #[serde(rename = "isActiveBroadcast")]
    pub is_active_broadcast: bool,

    #[serde(rename = "isEditing")]
    pub is_editing: bool,

    #[serde(rename = "isKollectiveAgentInstalled")]
    pub is_kollective_agent_installed: bool,

    #[serde(rename = "isEmbed")]
    pub is_embed: bool,

    #[serde(rename = "responseType")]
    pub response_type: String,
}
