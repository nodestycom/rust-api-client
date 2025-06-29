use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VpsAction {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "restart")]
    Restart,
    #[serde(rename = "poweroff")]
    PowerOff,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsBackup {
    pub date: String,
    pub file: String,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsChangePasswordData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsCpuDetails {
    pub manu: String,
    pub limit: u32,
    pub used: u32,
    pub free: u32,
    pub percent: f64,
    pub cores: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsRamDetails {
    pub limit: u64,
    pub used: u64,
    pub free: u64,
    pub percent: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsInodeDetails {
    pub limit: u64,
    pub used: u64,
    pub free: u64,
    pub percent: u32,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsNetworkSpeedDetails {
    #[serde(rename = "in")]
    pub network_in: u32,
    pub out: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsBandwidthOverall {
    pub usage: u64,
    #[serde(rename = "in")]
    pub bandwidth_in: u64,
    pub out: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsBandwidthGraphPoint {
    pub usage: u64,
    #[serde(rename = "in")]
    pub bandwidth_in: u64,
    pub out: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsVncDetails {
    pub enabled: bool,
    pub ip: String,
    pub port: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsOsInfo {
    pub name: String,
    pub distro: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsDiskDetails {
    pub limit: u64,
    pub used: u64,
    pub free: u64,
    pub percent: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsDetails {
    pub vps_id: u32,
    pub proxmox_id: u32,
    pub hostname: String,
    pub os_reinstall_limit: u32,
    pub status: bool,
    pub vnc: VpsVncDetails,
    pub os: VpsOsInfo,
    pub disk: VpsDiskDetails,
    pub ips: Vec<String>,
    pub cpu: VpsCpuDetails,
    pub ram: VpsRamDetails,
    pub inode: VpsInodeDetails,
    pub netspeed: VpsNetworkSpeedDetails,
    pub bandwidth: VpsBandwidthInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsBandwidthInfo {
    pub total: VpsBandwidthOverall,
    pub usage: Vec<u64>,
    #[serde(rename = "in")]
    pub bandwidth_in: Vec<u64>,
    pub out: Vec<u64>,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsGraphs {
    pub avg_download: u64,
    pub avg_upload: u64,
    pub avg_io_read: u64,
    pub avg_io_write: u64,
    pub cpu_usage: HashMap<String, f64>,
    pub inode_usage: HashMap<String, u64>,
    pub ram_usage: HashMap<String, u64>,
    pub disk_usage: HashMap<String, u64>,
    pub io_speed: VpsIoSpeed,
    pub network_speed: VpsNetworkSpeedGraph,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsIoSpeed {
    pub read: Vec<u64>,
    pub write: Vec<u64>,
    pub categories: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpsNetworkSpeedGraph {
    pub download: Vec<u64>,
    pub upload: Vec<u64>,
    pub categories: Vec<u64>,
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
    pub progress: String,
    pub started_at: u64,
    pub ended_at: u64,
}
