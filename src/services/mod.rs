pub mod dedicated;
pub mod firewall;
pub mod mail;
pub mod user;
pub mod vps;

pub use dedicated::DedicatedServerApiService;
pub use firewall::FirewallApiService;
pub use mail::MailHostingApiService;
pub use user::UserApiService;
pub use vps::VpsApiService;
