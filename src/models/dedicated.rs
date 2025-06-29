use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DedicatedServerAction {
    #[serde(rename = "setPowerOn")]
    Start,
    #[serde(rename = "setPowerOff")]
    Stop,
    #[serde(rename = "setPowerReset")]
    Restart,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerCpuDetails {
    pub model: String,
    pub speed: u32,
    pub turbo_speed: u32,
    pub cores: u8,
    pub threads: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerDetails {
    pub dedicated_id: String,
    pub status: bool,
    pub available_actions: Vec<DedicatedServerAction>,
    pub mainboard: String,
    pub ram: u32,
    pub disk: u32,
    pub cpu: DedicatedServerCpuDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerOsTemplate {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerReinstallData {
    pub password: String,
    pub os_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerTask {
    pub action: String,
    pub started_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerHardwareComponent {
    pub component: String,
    pub model: String,
    pub value: u32,
    pub value_suffix: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
#[repr(u8)]
pub enum DedicatedServerReinstallStep {
    RebootingServer = 0,
    PreparingBootEnvironment = 1,
    InstallingOperatingSystem = 2,
    InstallationCompleted = 3,
}

impl TryFrom<u8> for DedicatedServerReinstallStep {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DedicatedServerReinstallStep::RebootingServer),
            1 => Ok(DedicatedServerReinstallStep::PreparingBootEnvironment),
            2 => Ok(DedicatedServerReinstallStep::InstallingOperatingSystem),
            3 => Ok(DedicatedServerReinstallStep::InstallationCompleted),
            _ => Err("unknown DedicatedServerReinstallStep data"),
        }
    }
}

impl From<DedicatedServerReinstallStep> for u8 {
    fn from(step: DedicatedServerReinstallStep) -> Self {
        step as u8
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerReinstallStatus {
    pub completed: bool,
    pub step: DedicatedServerReinstallStep,
}
