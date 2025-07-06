use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackNotificationSettings {
    pub email_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord_webhook_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallAttackLog {
    pub started_at: u64,
    pub ended_at: u64,
    pub vectors: Vec<String>,
    pub peak: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallReverseDns {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdns: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRule {
    pub id: u32,
    pub protocol: String,
    pub service: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallCreateRuleData {
    pub port: u16,
    pub app_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallStatistics {
    pub timestamp: String,
    pub total_pass_traffic: String,
    pub total_drop_traffic: String,
}
