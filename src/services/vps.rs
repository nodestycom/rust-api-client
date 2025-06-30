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
use reqwest::Client;
use std::collections::HashMap;

pub struct VpsApiService {
    client: Client,
    base_url: String,
}

impl VpsApiService {
    pub fn new(client: Client, base_url: String) -> Self {
        Self {
            client,
            base_url,
        }
    }

    pub async fn perform_action(
        &self,
        id: &str,
        action: VpsAction,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/action", self.base_url, id);
        let mut body = HashMap::new();
        body.insert("action", action);

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }
    pub async fn restore_backup(
        &self,
        id: &str,
        data: &VpsBackup,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/backup/restore", self.base_url, id);
        let response = self
            .client
            .post(&url)
            .json(&data)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }

    pub async fn get_backups(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<VpsBackup>>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/backups", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<Vec<VpsBackup>>>().await
    }

    pub async fn get_bandwidth_graphs(
        &self,
        id: &str,
    ) -> Result<ApiResponse<VpsGraphs>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/graphs/bandwidth", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<VpsGraphs>>().await
    }

    pub async fn get_cpu_graphs(
        &self,
        id: &str,
    ) -> Result<ApiResponse<VpsGraphs>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/graphs/cpu", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<VpsGraphs>>().await
    }

    pub async fn get_io_graphs(&self, id: &str) -> Result<ApiResponse<VpsGraphs>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/graphs/io", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<VpsGraphs>>().await
    }

    pub async fn get_ram_graphs(
        &self,
        id: &str,
    ) -> Result<ApiResponse<VpsGraphs>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/graphs/ram", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<VpsGraphs>>().await
    }

    pub async fn change_password(
        &self,
        id: &str,
        data: VpsChangePasswordData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/change-password", self.base_url, id);
        let response = self
            .client
            .post(&url)
            .json(&data)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }

    pub async fn get_details(&self, id: &str) -> Result<ApiResponse<VpsDetails>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/details", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<VpsDetails>>().await
    }

    pub async fn get_os_templates(
        &self,
        id: &str,
    ) -> Result<ApiResponse<Vec<VpsOsTemplate>>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/os-templates", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<Vec<VpsOsTemplate>>>().await
    }

    pub async fn reinstall(
        &self,
        id: &str,
        data: VpsReinstallData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/reinstall", self.base_url, id);
        let response = self
            .client
            .post(&url)
            .json(&data)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }

    pub async fn get_tasks(&self, id: &str) -> Result<ApiResponse<Vec<VpsTask>>, reqwest::Error> {
        let url = format!("{}/services/{}/vps/tasks", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<Vec<VpsTask>>>().await
    }
}