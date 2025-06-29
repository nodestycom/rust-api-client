use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: u32,
    pub product_id: u32,
    pub group_id: u32,
    pub name: String,
    pub raw_name: String,
    pub domain: String,
    pub first_payment_amount: f64,
    pub recurring_amount: f64,
    pub billing_cycle: String,
    pub next_due_date: u64,
    pub status: String,
    pub username: String,
    pub password: Option<String>,
    pub vps_id: Option<u32>,
    pub dedicated_id: Option<Vec<String>>,
    pub is_vps: bool,
    pub is_web_hosting: bool,
    pub is_dedicated: bool,
    pub is_hetzner_dedicated: bool,
    pub is_sky_link_dedicated: bool,
    pub addons: Vec<Addon>,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Addon {
    pub name: String,
    pub recurring_amount: f64,
    pub billing_cycle: String,
    pub status: String,
    pub register_date: u64,
    pub next_due_date: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TicketStatus {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "ANSWERED")]
    Answered,
    #[serde(rename = "CLOSED")]
    Closed,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketAuthor {
    pub id: String,
    pub avatar: Option<String>,
    pub name: String,
    pub role: String,
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
    pub author: TicketAuthor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: String,
    pub subject: String,
    pub status: TicketStatus,
    pub priority: String,
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
    pub priority: String,
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
    pub tckn: Option<String>,
    pub birth_year: Option<String>,
    pub banned: bool,
    pub current_session_id: String,
    pub totp_enabled: bool,
    pub stats: UserStats,
    pub company_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum InvoiceStatus {
    #[serde(rename = "Paid")]
    Paid,
    #[serde(rename = "Unpaid")]
    Unpaid,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Refunded")]
    Refunded,
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
