use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum BillingCycle {
    #[serde(rename = "Monthly")]
    Monthly,
    #[serde(rename = "Quarterly")]
    Quarterly,
    #[serde(rename = "Semi-Annually")]
    SemiAnnually,
    #[serde(rename = "Annually")]
    Annually,
    #[serde(rename = "Biennially")]
    Biennially,
    #[serde(rename = "Triennially")]
    Triennially,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ServiceStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Suspended")]
    Suspended,
    #[serde(rename = "Terminated")]
    Terminated,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Fraud")]
    Fraud,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAddon {
    pub name: String,
    pub recurring_amount: f64,
    pub billing_cycle: BillingCycle,
    pub status: ServiceStatus,
    pub register_date: u64,
    pub next_due_date: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum DedicatedServerLocation {
    #[serde(rename = "FRA-01")]
    Fra01,
    #[serde(rename = "FRA-02")]
    Fra02,
    #[serde(rename = "FRA-03")]
    Fra03,
    #[serde(rename = "USA-01")]
    Usa01,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: u32,
    pub product_id: u32,
    #[serde(default)]
    pub group_id: Option<u32>,
    pub name: String,
    pub raw_name: String,
    pub name_without_group_name: String,
    pub domain: String,
    pub first_payment_amount: f64,
    pub recurring_amount: f64,
    pub billing_cycle: BillingCycle,
    pub next_due_date: u64,
    pub status: ServiceStatus,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub vps_id: Option<u32>,
    #[serde(default)]
    pub dedicated_id: Option<String>,
    pub is_vps: bool,
    pub is_web_hosting: bool,
    pub is_dedicated: bool,
    pub is_mail_hosting: bool,
    #[serde(default)]
    pub dedicated_server_location: Option<DedicatedServerLocation>,
    pub addons: Vec<ServiceAddon>,
    pub features: Vec<String>,
    pub ips: Vec<String>,
    #[serde(default)]
    pub team_id: Option<u32>,
    pub owner: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TicketStatus {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "CLOSED")]
    Closed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TicketPriority {
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TicketMessageAuthorRole {
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "ADMIN")]
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketMessageAuthor {
    pub id: String,
    #[serde(default)]
    pub avatar: Option<String>,
    pub name: String,
    pub role: TicketMessageAuthorRole,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketMessage {
    pub id: String,
    pub message_id: String,
    pub content: String,
    pub attachments: Vec<String>,
    pub author_id: String,
    pub created_at: String,
    pub author: TicketMessageAuthor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: String,
    pub subject: String,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub last_reply: String,
    pub marked: bool,
    pub messages: Vec<TicketMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTicketSummary {
    pub id: String,
    pub subject: String,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub last_reply: String,
    pub marked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    pub active_services: u32,
    pub unpaid_invoices: u32,
    pub balance: f64,
    pub active_tickets: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub email: String,
    pub country: String,
    pub city: String,
    pub state: String,
    pub address: String,
    pub post_code: String,
    pub currency: String,
    pub currency_symbol: String,
    pub phone_number: String,
    pub tckn: String,
    pub birth_year: String,
    pub banned: bool,
    pub current_session_id: String,
    pub totp_enabled: bool,
    pub stats: UserStats,
    #[serde(default)]
    pub company_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum InvoiceStatus {
    #[serde(rename = "Draft")]
    Draft,
    #[serde(rename = "Paid")]
    Paid,
    #[serde(rename = "Unpaid")]
    Unpaid,
    #[serde(rename = "Overdue")]
    Overdue,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Refunded")]
    Refunded,
    #[serde(rename = "Payment Pending")]
    PaymentPending,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceItem {
    pub id: u32,
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: u32,
    pub due_date: u64,
    #[serde(default)]
    pub date_paid: Option<u64>,
    pub sub_total: f64,
    pub total: f64,
    pub status: InvoiceStatus,
    pub applied_balance: f64,
    pub items: Vec<InvoiceItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInvoiceSummary {
    pub id: u32,
    pub due_date: u64,
    #[serde(default)]
    pub date_paid: Option<u64>,
    pub sub_total: f64,
    pub total: f64,
    pub status: InvoiceStatus,
    pub applied_balance: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum SessionOs {
    #[serde(rename = "Desktop")]
    Desktop,
    #[serde(rename = "Mobile")]
    Mobile,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    pub ip: String,
    pub location: String,
    pub os: SessionOs,
    pub platform: String,
    pub last_seen: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReferralCode {
    pub code: String,
    pub uses: UserReferralCodeUses,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReferralCodeUses {
    pub count: u32,
    pub data: Vec<UserReferralCodeUse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReferralCodeUse {
    pub date: u64,
    pub amount: f64,
}
