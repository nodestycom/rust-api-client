use crate::models::RestClientOptions;
use crate::ApiResponse;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, ClientBuilder, Error, Method};
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;

pub struct NodestyApiClient {
    client: Client,
    base_url: String,
}

impl NodestyApiClient {
    pub fn new(options: RestClientOptions) -> Result<Arc<Self>, Error> {
        let auth_header = HeaderValue::from_str(&format!("PAT {}", options.access_token))
            .expect("Invalid authorization token");

        let timeout = Duration::from_millis(options.timeout_ms.unwrap_or(30_000));

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_header);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .timeout(timeout)
            .connect_timeout(timeout)
            .build()?;

        Ok(Arc::new(Self {
            client,
            base_url: options.base_url.clone(),
        }))
    }

    pub async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<ApiResponse<T>, Error> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.request(method, &url);

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let status = response.status();
        let raw_data: Value = response.json().await?;

        let api_response = if !status.is_success() {
            ApiResponse {
                success: false,
                error: raw_data["message"].as_str().map(|s| s.to_string()),
                data: None,
            }
        } else {
            ApiResponse {
                success: true,
                error: None,
                data: serde_json::from_value(raw_data).ok(),
            }
        };

        Ok(api_response)
    }
}