use crate::models::{
    user::{
        Invoice,
        Service,
        Session,
        Ticket,
        User,
        UserInvoiceSummary,
        UserTicketSummary
    },
    ApiResponse,
};
use reqwest::Client;

pub struct UserApiService {
    client: Client,
    base_url: String,
    access_token: String,
}

impl UserApiService {
    pub fn new(client: Client, base_url: String, access_token: String) -> Self {
        Self {
            client,
            base_url,
            access_token,
        }
    }

    pub async fn get_services(&self) -> Result<ApiResponse<Vec<Service>>, reqwest::Error> {
        let url = format!("{}/services", self.base_url);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response.json::<ApiResponse<Vec<Service>>>().await
    }

    pub async fn get_ticket_by_id(
        &self,
        ticket_id: &str,
    ) -> Result<ApiResponse<Ticket>, reqwest::Error> {
        let url = format!("{}/tickets/{}", self.base_url, ticket_id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response.json::<ApiResponse<Ticket>>().await
    }

    pub async fn get_tickets(&self) -> Result<ApiResponse<Vec<UserTicketSummary>>, reqwest::Error> {
        let url = format!("{}/tickets", self.base_url);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response.json::<ApiResponse<Vec<UserTicketSummary>>>().await
    }

    pub async fn get_current_user(&self) -> Result<ApiResponse<User>, reqwest::Error> {
        let url = format!("{}/users/@me", self.base_url);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response.json::<ApiResponse<User>>().await
    }

    pub async fn get_invoice_by_id(
        &self,
        invoice_id: &str,
    ) -> Result<ApiResponse<Invoice>, reqwest::Error> {
        let url = format!("{}/users/@me/invoices/{}", self.base_url, invoice_id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response.json::<ApiResponse<Invoice>>().await
    }

    pub async fn get_invoices(
        &self,
    ) -> Result<ApiResponse<Vec<UserInvoiceSummary>>, reqwest::Error> {
        let url = format!("{}/users/@me/invoices", self.base_url);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response
            .json::<ApiResponse<Vec<UserInvoiceSummary>>>()
            .await
    }

    pub async fn get_sessions(&self) -> Result<ApiResponse<Vec<Session>>, reqwest::Error> {
        let url = format!("{}/users/@me/sessions", self.base_url);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response.json::<ApiResponse<Vec<Session>>>().await
    }
}
