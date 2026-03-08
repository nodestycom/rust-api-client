use crate::NodestyApiClient;
use crate::models::{
    ApiResponse,
    vps::{
        VpsAction, VpsBackup, VpsChangePasswordData, VpsDailyBackupStatusData, VpsDetails,
        VpsOsTemplate, VpsReinstallData, VpsTask, VpsUpdateBackupData, VpsUsageGraphEntry,
    },
};
use reqwest::Method;
use std::sync::Arc;

pub struct VpsApiService {
    client: Arc<NodestyApiClient>,
}

impl VpsApiService {
    pub fn new(client: Arc<NodestyApiClient>) -> Self {
        Self { client }
    }

    pub async fn perform_action(
        &self,
        id: &str,
        action: VpsAction,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let body = serde_json::json!({ "action": action });
        self.client
            .send_request(
                Method::POST,
                &format!("/services/{id}/vps/action"),
                Some(body),
            )
            .await
    }

    pub async fn restore_backup(
        &self,
        id: &str,
        file: &str,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        self.client
            .send_request(
                Method::POST,
                &format!("/services/{id}/vps/backups/{file}/restore"),
                None,
            )
            .await
    }

    pub async fn update_backup(
        &self,
        id: &str,
        file: &str,
        data: VpsUpdateBackupData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let body = serde_json::to_value(&data).ok();
        self.client
            .send_request(
                Method::PATCH,
                &format!("/services/{id}/vps/backups/{file}"),
                body,
            )
            .await
    }

    pub async fn get_backups(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<VpsBackup>>, reqwest::Error> {
        self.client
            .send_request(Method::GET, &format!("/services/{id}/vps/backups"), None)
            .await
    }

    pub async fn create_backup(&self, id: &str) -> Result<ApiResponse<()>, reqwest::Error> {
        self.client
            .send_request(Method::POST, &format!("/services/{id}/vps/backups"), None)
            .await
    }

    pub async fn change_daily_backup_status(
        &self,
        id: &str,
        data: VpsDailyBackupStatusData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let body = serde_json::to_value(&data).ok();
        self.client
            .send_request(
                Method::PUT,
                &format!("/services/{id}/vps/backups/daily-backups"),
                body,
            )
            .await
    }

    pub async fn change_password(
        &self,
        id: &str,
        data: VpsChangePasswordData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let body = serde_json::to_value(&data).ok();
        self.client
            .send_request(
                Method::POST,
                &format!("/services/{id}/vps/change-password"),
                body,
            )
            .await
    }

    pub async fn get_usage_statistics(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<VpsUsageGraphEntry>>, reqwest::Error> {
        self.client
            .send_request(Method::GET, &format!("/services/{id}/vps/graphs"), None)
            .await
    }

    pub async fn get_details(&self, id: &str) -> Result<ApiResponse<VpsDetails>, reqwest::Error> {
        self.client
            .send_request(Method::GET, &format!("/services/{id}/vps/info"), None)
            .await
    }

    pub async fn get_os_templates(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<VpsOsTemplate>>, reqwest::Error> {
        self.client
            .send_request(
                Method::GET,
                &format!("/services/{id}/vps/os-templates"),
                None,
            )
            .await
    }

    pub async fn reinstall(
        &self,
        id: &str,
        data: VpsReinstallData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let body = serde_json::to_value(&data).ok();
        self.client
            .send_request(Method::POST, &format!("/services/{id}/vps/reinstall"), body)
            .await
    }

    pub async fn get_tasks(&self, id: &str) -> Result<ApiResponse<Vec<VpsTask>>, reqwest::Error> {
        self.client
            .send_request(Method::GET, &format!("/services/{id}/vps/tasks"), None)
            .await
    }
}
