use crate::models::{
    dedicated::{
        DedicatedServerAction,
        DedicatedServerHardwareComponent,
        DedicatedServerOsTemplate,
        DedicatedServerReinstallData,
        DedicatedServerReinstallStatus,
        DedicatedServerTask,
    },
    ApiResponse,
};
use reqwest::Client;
use std::collections::HashMap;
pub struct DedicatedServerApiService {
    client: Client,
    base_url: String,
    access_token: String,
}

impl DedicatedServerApiService {
    pub fn new(client: Client, base_url: String, access_token: String) -> Self {
        Self {
            client,
            base_url,
            access_token,
        }
    }

    pub async fn perform_action(
        &self,
        id: &str,
        action: DedicatedServerAction,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!("{}/services/{}/dedicated/action", self.base_url, id);

        let mut body = HashMap::new();
        body.insert("action", serde_json::to_value(action).unwrap());

        let response = self
            .client
            .post(&url)
            .bearer_auth(self.access_token.clone())
            .json(&body)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }
    pub async fn get_hardware_components(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<DedicatedServerHardwareComponent>>, reqwest::Error> {
        let url = format!("{}/services/{}/dedicated/hardware", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response
            .json::<ApiResponse<Vec<DedicatedServerHardwareComponent>>>()
            .await
    }

    pub async fn get_os_templates(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<DedicatedServerOsTemplate>>, reqwest::Error> {
        let url = format!("{}/services/{}/dedicated/os-templates", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response
            .json::<ApiResponse<Vec<DedicatedServerOsTemplate>>>()
            .await
    }

    pub async fn get_reinstall_status(
        &self,
        id: &str,
    ) -> Result<ApiResponse<DedicatedServerReinstallStatus>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/dedicated/reinstall-status",
            self.base_url, id
        );
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response
            .json::<ApiResponse<DedicatedServerReinstallStatus>>()
            .await
    }

    pub async fn reinstall(
        &self,
        id: &str,
        data: DedicatedServerReinstallData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!("{}/services/{}/dedicated/reinstall", self.base_url, id);
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.access_token.clone())
            .json(&data)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }

    pub async fn get_tasks(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<DedicatedServerTask>>, reqwest::Error> {
        let url = format!("{}/services/{}/dedicated/tasks", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response
            .json::<ApiResponse<Vec<DedicatedServerTask>>>()
            .await
    }
}
