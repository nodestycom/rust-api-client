use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VpsAction {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "reset")]
    Reset,
    #[serde(rename = "stop")]
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsBackup {
    pub file: String,
    pub notes: String,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsUpdateBackupData {
    #[serde(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsDailyBackupStatusData {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsChangePasswordData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsUsageGraphEntry {
    pub net_out: u64,
    pub net_in: u64,
    pub ram_usage: u64,
    pub cpu_usage: f64,
    pub disk_read: u64,
    pub disk_write: u64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsVncDetails {
    pub port: String,
    pub ticket: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsOsDetails {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsCpuDetails {
    pub percent: f64,
    pub cores: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsRamUsage {
    pub limit: u64,
    pub used: u64,
    pub free: u64,
    pub percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsDetails {
    pub vps_id: u32,
    pub proxmox_id: u32,
    pub hostname: String,
    pub status: bool,
    pub vnc: VpsVncDetails,
    pub os: VpsOsDetails,
    pub disk: u64,
    pub ips: Vec<String>,
    pub cpu: VpsCpuDetails,
    pub ram: VpsRamUsage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsOsTemplate {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsReinstallData {
    pub password: String,
    pub os_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsTask {
    pub action: String,
    pub status: String,
    #[serde(default)]
    pub ended_at: Option<u64>,
    pub started_at: u64,
}
