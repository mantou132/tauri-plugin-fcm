use serde::{Deserialize, Serialize};

/// The last tapped notification, taken once by JS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenedNotification {
    /// The message `data`; absent when nothing was tapped since the last take.
    pub data: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FcmToken {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionState {
    Granted,
    Denied,
    Prompt,
    PromptWithRationale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionStatus {
    pub notification: PermissionState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChannelArgs {
    pub id: String,
    pub name: String,
    pub importance: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendNotificationArgs {
    pub title: String,
    pub body: Option<String>,
    pub icon: Option<String>,
    pub id: Option<i32>,
    pub channel_id: Option<String>,
}
