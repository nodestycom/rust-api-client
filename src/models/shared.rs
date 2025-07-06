use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    #[serde(default = "default_success")]
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

fn default_success() -> bool {
    true
}

#[derive(Debug, Clone)]
pub struct RestClientOptions {
    pub access_token: String,
    pub base_url: String,
    pub retry: Option<u32>,
    pub timeout_ms: Option<u64>,
    pub rate_limit_offset_ms: Option<u64>,
}

impl RestClientOptions {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            base_url: "https://nodesty.com/api".to_string(),
            retry: Some(3),
            timeout_ms: Some(30_000),
            rate_limit_offset_ms: Some(50),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn with_retry(mut self, retry: u32) -> Self {
        self.retry = Some(retry);
        self
    }

    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }

    pub fn with_rate_limit_offset_ms(mut self, rate_limit_offset_ms: u64) -> Self {
        self.rate_limit_offset_ms = Some(rate_limit_offset_ms);
        self
    }
}
