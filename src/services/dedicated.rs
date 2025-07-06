use crate::models::{
    dedicated::{
        DedicatedServerAction,
        DedicatedServerDetails,
        DedicatedServerHardwareComponent,
        DedicatedServerOsTemplate,
        DedicatedServerReinstallData,
        DedicatedServerReinstallStatus,
        DedicatedServerTask,
    },
    ApiResponse,
};
use crate::NodestyApiClient;
use reqwest::{Error, Method};
use std::sync::Arc;

pub struct DedicatedServerApiService {
    client: Arc<NodestyApiClient>,
}

impl DedicatedServerApiService {
    pub fn new(client: Arc<NodestyApiClient>) -> Self {
        Self {
            client,
        }
    }

    pub async fn perform_action(
        &self,
        id: &str,
        action: DedicatedServerAction,
    ) -> Result<ApiResponse<()>, Error> {
        let body = serde_json::json!({ "action": action });
        self.client.send_request(Method::POST, &format!("/services/{}/dedicated/action", id), Some(body)).await
    }

    pub async fn get_details(
        &self,
        id: &str,
    ) -> Result<ApiResponse<DedicatedServerDetails>, Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/dedicated/info", id), None).await
    }

    pub async fn get_hardware_components(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<DedicatedServerHardwareComponent>>, Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/dedicated/hardware", id), None).await
    }

    pub async fn get_os_templates(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<DedicatedServerOsTemplate>>, Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/dedicated/os-templates", id), None).await
    }

    pub async fn get_reinstall_status(
        &self,
        id: &str,
    ) -> Result<ApiResponse<DedicatedServerReinstallStatus>, Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/dedicated/reinstall-status", id), None).await
    }

    pub async fn reinstall(
        &self,
        id: &str,
        data: DedicatedServerReinstallData,
    ) -> Result<ApiResponse<()>, Error> {
        let body = serde_json::to_value(&data).ok();
        self.client.send_request(Method::POST, &format!("/services/{}/dedicated/reinstall", id), body).await
    }

    pub async fn get_tasks(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<DedicatedServerTask>>, Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/dedicated/tasks", id), None).await
    }
}