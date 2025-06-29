use reqwest::{Client, ClientBuilder};
use std::time::Duration;

use crate::services::{
    DedicatedServerApiService,
    FirewallApiService,
    UserApiService,
    VpsApiService,
};
const API_BASE_URL: &str = "https://nodesty.com/api";

#[derive(Debug, Clone)]
pub struct RestClientOptions {
    pub access_token: String,
    pub retry: u32,
    pub timeout: Duration,
    pub rate_limit_offset: u64,
}

impl Default for RestClientOptions {
    fn default() -> Self {
        Self {
            access_token: String::new(),
            retry: 3,
            timeout: Duration::from_secs(30),
            rate_limit_offset: 50,
        }
    }
}

impl RestClientOptions {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            ..Default::default()
        }
    }

    pub fn with_retry(mut self, retry: u32) -> Self {
        self.retry = retry;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_rate_limit_offset(mut self, offset: u64) -> Self {
        self.rate_limit_offset = offset;
        self
    }
}

pub struct NodestyApiClient {
    #[allow(dead_code)]
    reqwest_client: Client,
    base_url: String,
    access_token: String,

    pub user: UserApiService,
    pub vps: VpsApiService,
    pub firewall: FirewallApiService,
    pub dedicated_server: DedicatedServerApiService,
}

impl NodestyApiClient {
    pub fn new(options: RestClientOptions) -> Self {
        let client_builder = ClientBuilder::new()
            .timeout(options.timeout)
            .connect_timeout(options.timeout);

        let reqwest_client = client_builder
            .build()
            .expect("Failed to build client");

        let base_url = API_BASE_URL.to_string();
        let access_token = options.access_token;

        Self {
            user: UserApiService::new(
                reqwest_client.clone(),
                base_url.clone(),
                access_token.clone(),
            ),
            vps: VpsApiService::new(
                reqwest_client.clone(),
                base_url.clone(),
                access_token.clone(),
            ),
            firewall: FirewallApiService::new(
                reqwest_client.clone(),
                base_url.clone(),
                access_token.clone(),
            ),
            dedicated_server: DedicatedServerApiService::new(
                reqwest_client.clone(),
                base_url.clone(),
                access_token.clone(),
            ),
            reqwest_client,
            base_url,
            access_token,
        }
    }
}
