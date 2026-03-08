use crate::ApiResponse;
use crate::models::RestClientOptions;
use crate::services::{
    DedicatedServerApiService, FirewallApiService, MailHostingApiService, UserApiService,
    VpsApiService,
};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, Error, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

pub struct NodestyApiClient {
    client: Client,
    base_url: String,
    retry: u32,
    rate_limit_offset_ms: u64,
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
            base_url: options.base_url,
            retry: options.retry.unwrap_or(3),
            rate_limit_offset_ms: options.rate_limit_offset_ms.unwrap_or(50),
        }))
    }

    pub fn user(self: &Arc<Self>) -> UserApiService {
        UserApiService::new(self.clone())
    }

    pub fn vps(self: &Arc<Self>) -> VpsApiService {
        VpsApiService::new(self.clone())
    }

    pub fn firewall(self: &Arc<Self>) -> FirewallApiService {
        FirewallApiService::new(self.clone())
    }

    pub fn dedicated_server(self: &Arc<Self>) -> DedicatedServerApiService {
        DedicatedServerApiService::new(self.clone())
    }

    pub fn mail_hosting(self: &Arc<Self>) -> MailHostingApiService {
        MailHostingApiService::new(self.clone())
    }

    fn should_retry_status(status: StatusCode) -> bool {
        matches!(
            status.as_u16(),
            408 | 409 | 425 | 429 | 500 | 502 | 503 | 504
        )
    }

    fn retry_delay(response: &Response, rate_limit_offset_ms: u64) -> Duration {
        let delay_ms = response
            .headers()
            .get("x-ratelimit-reset")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .map(|reset_at| {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64;

                reset_at.saturating_sub(now) + rate_limit_offset_ms
            })
            .unwrap_or(1_000);

        Duration::from_millis(delay_ms)
    }

    fn has_api_error(raw_data: &Value) -> bool {
        raw_data
            .get("error")
            .map(|value| match value {
                Value::Bool(flag) => *flag,
                Value::Null => false,
                Value::String(message) => !message.is_empty(),
                _ => true,
            })
            .unwrap_or(false)
    }

    fn extract_error_message(raw_data: &Value, fallback_status: StatusCode) -> Option<String> {
        raw_data
            .get("message")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .or_else(|| {
                raw_data
                    .get("error")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            })
            .or_else(|| raw_data.as_str().map(str::to_owned))
            .or_else(|| fallback_status.canonical_reason().map(str::to_owned))
    }

    fn is_empty_payload(raw_data: &Value) -> bool {
        raw_data.is_null() || matches!(raw_data, Value::Object(map) if map.is_empty())
    }

    pub async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<ApiResponse<T>, Error> {
        let url = format!("{}{}", self.base_url, path);
        let mut attempts = 0;

        loop {
            let mut request = self.client.request(method.clone(), &url);

            if let Some(body) = body.as_ref() {
                request = request.json(body);
            }

            let response = match request.send().await {
                Ok(response) => response,
                Err(error) if attempts < self.retry => {
                    attempts += 1;
                    sleep(Duration::from_millis(1_000)).await;
                    continue;
                }
                Err(error) => return Err(error),
            };

            let status = response.status();

            if Self::should_retry_status(status) && attempts < self.retry {
                attempts += 1;
                sleep(Self::retry_delay(&response, self.rate_limit_offset_ms)).await;
                continue;
            }

            let raw_data: Value = response.json().await?;

            if !status.is_success() || Self::has_api_error(&raw_data) {
                return Ok(ApiResponse {
                    success: false,
                    error: Self::extract_error_message(&raw_data, status),
                    data: None,
                });
            }

            return match serde_json::from_value(raw_data.clone()) {
                Ok(data) => Ok(ApiResponse {
                    success: true,
                    error: None,
                    data: Some(data),
                }),
                Err(_) if Self::is_empty_payload(&raw_data) => Ok(ApiResponse {
                    success: true,
                    error: None,
                    data: None,
                }),
                Err(error) => Ok(ApiResponse {
                    success: false,
                    error: Some(format!("Failed to deserialize response body: {error}")),
                    data: None,
                }),
            };
        }
    }
}
