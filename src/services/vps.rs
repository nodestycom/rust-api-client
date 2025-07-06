use crate::models::{
    vps::{
        VpsAction,
        VpsBackup,
        VpsChangePasswordData,
        VpsDetails,
        VpsGraphs,
        VpsOsTemplate,
        VpsReinstallData,
        VpsTask,
    },
    ApiResponse,
};
use crate::NodestyApiClient;
use reqwest::Method;
use std::sync::Arc;

pub struct VpsApiService {
    client: Arc<NodestyApiClient>,
}

impl VpsApiService {
    pub fn new(client: Arc<NodestyApiClient>) -> Self {
        Self {
            client,
        }
    }

    pub async fn perform_action(
        &self,
        id: &str,
        action: VpsAction,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let body = serde_json::json!({ "action": action });
        self.client.send_request(Method::POST, &format!("/services/{}/vps/action", id), Some(body)).await
    }

    pub async fn restore_backup(
        &self,
        id: &str,
        data: &VpsBackup,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        self.client.send_request(Method::POST, &format!("/services/{}/vps/backups/{}/{}", id, data.date, data.file), None).await
    }

    pub async fn get_backups(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<VpsBackup>>, reqwest::Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/vps/backups", id), None).await
    }

    pub async fn change_password(
        &self,
        id: &str,
        data: VpsChangePasswordData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let body = serde_json::to_value(&data).ok();
        self.client.send_request(Method::POST, &format!("/services/{}/vps/change-password", id), body).await
    }

    pub async fn get_usage_statistics(
        &self,
        id: &str,
    ) -> Result<ApiResponse<VpsGraphs>, reqwest::Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/vps/graphs", id), None).await
    }

    pub async fn get_details(
        &self,
        id: &str,
    ) -> Result<ApiResponse<VpsDetails>, reqwest::Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/vps/info", id), None).await
    }

    pub async fn get_os_templates(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<VpsOsTemplate>>, reqwest::Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/vps/os-templates", id), None).await
    }

    pub async fn reinstall(
        &self,
        id: &str,
        data: VpsReinstallData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let body = serde_json::to_value(&data).ok();
        self.client.send_request(Method::POST, &format!("/services/{}/vps/reinstall", id), body).await
    }

    pub async fn get_tasks(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<VpsTask>>, reqwest::Error> {
        self.client.send_request(Method::GET, &format!("/services/{}/vps/tasks", id), None).await
    }
}