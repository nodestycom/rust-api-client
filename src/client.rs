use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, ClientBuilder};
use std::time::Duration;


use crate::services::{
    DedicatedServerApiService,
    FirewallApiService,
    UserApiService,
    VpsApiService,
};

use crate::models::RestClientOptions;

pub struct NodestyApiClient {

    #[allow(dead_code)]
    reqwest_client: Client,
    base_url: String,

    pub user: UserApiService,
    pub vps: VpsApiService,
    pub firewall: FirewallApiService,
    pub dedicated_server: DedicatedServerApiService,
}

impl NodestyApiClient {
    pub fn new(options: RestClientOptions) -> Result<Self, reqwest::Error> {

        let auth_header_string = format!("PAT {}", options.access_token);

        let auth_header_value = HeaderValue::from_str(&auth_header_string).expect("invalid auth header");

        let timeout_duration = Duration::from_millis(options.timeout_ms.unwrap_or(30_000));

        let mut headers = HeaderMap::new();

        headers.insert(AUTHORIZATION, auth_header_value);

        let client_builder = ClientBuilder::new()
            .timeout(timeout_duration)
            .connect_timeout(timeout_duration)
            .default_headers(headers);

        let reqwest_client = client_builder
            .build()?;

        let base_url = options.base_url;

        Ok(Self {
            user: UserApiService::new(
                reqwest_client.clone(),
                base_url.clone(),
            ),
            vps: VpsApiService::new(
                reqwest_client.clone(),
                base_url.clone(),
            ),
            firewall: FirewallApiService::new(
                reqwest_client.clone(),
                base_url.clone(),
            ),
            dedicated_server: DedicatedServerApiService::new(
                reqwest_client.clone(),
                base_url.clone(),
            ),
            reqwest_client,
            base_url,
        })
    }
}