use reqwest::{Error, Method};
use std::sync::Arc;

use crate::models::{
    user::{
        Invoice,
        Service,
        Session,
        Ticket,
        User,
        UserInvoiceSummary,
        UserTicketSummary,
    },
    ApiResponse,
};

use crate::NodestyApiClient;

pub struct UserApiService {
    client: Arc<NodestyApiClient>,
}

impl UserApiService {
    pub fn new(client: Arc<NodestyApiClient>) -> Self {
        Self {
            client
        }
    }
    pub async fn get_services(&self) -> Result<ApiResponse<Vec<Service>>, Error> {
        self.client.send_request(Method::GET, "/services", None).await
    }

    pub async fn get_ticket_by_id(
        &self,
        ticket_id: &str,
    ) -> Result<ApiResponse<Ticket>, Error> {
        self.client.send_request(Method::GET, &format!("/tickets/{}", ticket_id), None).await
    }

    pub async fn get_tickets(&self) -> Result<ApiResponse<Vec<UserTicketSummary>>, Error> {
        self.client.send_request(Method::GET, "/tickets", None).await
    }

    pub async fn get_current_user(&self) -> Result<ApiResponse<User>, Error> {
        self.client.send_request(Method::GET, "/users/@me", None).await
    }

    pub async fn get_invoice_by_id(
        &self,
        invoice_id: &str,
    ) -> Result<ApiResponse<Invoice>, Error> {
        self.client.send_request(Method::GET, &format!("/users/@me/invoices/{}", invoice_id), None).await
    }

    pub async fn get_invoices(&self) -> Result<ApiResponse<Vec<UserInvoiceSummary>>, Error> {
        self.client.send_request(Method::GET, "/users/@me/invoices", None).await
    }

    pub async fn get_sessions(&self) -> Result<ApiResponse<Vec<Session>>, Error> {
        self.client.send_request(Method::GET, "/users/@me/sessions", None).await
    }
}