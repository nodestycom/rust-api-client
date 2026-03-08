use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum MailHostingStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHostingCountLimit {
    pub count: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHostingDiskUsage {
    pub usage: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHostingUsers {
    pub count: u32,
    pub limit: u32,
    pub alias_count: u32,
    pub alias_limit: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum MailHostingArchiveYearsStatus {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHostingArchiveYears {
    pub number: u32,
    pub status: MailHostingArchiveYearsStatus,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum MailHostingVerificationRecordType {
    #[serde(rename = "TXT")]
    Txt,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHostingVerification {
    pub status: bool,
    #[serde(rename = "type")]
    pub record_type: MailHostingVerificationRecordType,
    pub record: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum MailHostingDnsRecordType {
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "CNAME")]
    Cname,
    #[serde(rename = "TXT")]
    Txt,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHostingDnsRecord {
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: MailHostingDnsRecordType,
    pub value: String,
    pub priority: u32,
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHostingDkimRecord {
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub value: String,
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHostingDetails {
    pub status: MailHostingStatus,
    pub spam_experts: bool,
    pub file_storage: bool,
    pub office: bool,
    pub domain_alias: MailHostingCountLimit,
    pub disk: MailHostingDiskUsage,
    pub users: MailHostingUsers,
    pub archive_years: MailHostingArchiveYears,
    pub verified: MailHostingVerification,
    pub dns: Vec<MailHostingDnsRecord>,
    #[serde(default)]
    pub dkim: Option<MailHostingDkimRecord>,
}
