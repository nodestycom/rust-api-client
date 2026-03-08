use crate::NodestyApiClient;
use crate::models::{ApiResponse, mail::MailHostingDetails};
use reqwest::{Error, Method};
use std::sync::Arc;

pub struct MailHostingApiService {
    client: Arc<NodestyApiClient>,
}

impl MailHostingApiService {
    pub fn new(client: Arc<NodestyApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_details(&self, id: &str) -> Result<ApiResponse<MailHostingDetails>, Error> {
        self.client
            .send_request(Method::GET, &format!("/services/{id}/mail/info"), None)
            .await
    }
}
