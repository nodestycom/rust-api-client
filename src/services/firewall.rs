use crate::NodestyApiClient;
use crate::models::{
    ApiResponse,
    firewall::{
        AttackNotificationSettings, FirewallAttackLog, FirewallReverseDns, FirewallRule,
        FirewallStatistics,
    },
};
use reqwest::{Error, Method};
use std::sync::Arc;

pub struct FirewallApiService {
    client: Arc<NodestyApiClient>,
}

impl FirewallApiService {
    pub fn new(client: Arc<NodestyApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_attack_logs(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<Vec<FirewallAttackLog>>, Error> {
        self.client
            .send_request(
                Method::GET,
                &format!("/services/{service_id}/firewall/{ip}/attack-logs"),
                None,
            )
            .await
    }

    pub async fn get_attack_notification_settings(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<AttackNotificationSettings>, Error> {
        self.client
            .send_request(
                Method::GET,
                &format!("/services/{service_id}/firewall/{ip}/attack-notification"),
                None,
            )
            .await
    }

    pub async fn update_attack_notification_settings(
        &self,
        service_id: &str,
        ip: &str,
        data: AttackNotificationSettings,
    ) -> Result<ApiResponse<AttackNotificationSettings>, Error> {
        let body = serde_json::to_value(&data).ok();
        self.client
            .send_request(
                Method::PUT,
                &format!("/services/{service_id}/firewall/{ip}/attack-notification"),
                body,
            )
            .await
    }

    pub async fn reset_reverse_dns(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<()>, Error> {
        self.client
            .send_request(
                Method::DELETE,
                &format!("/services/{service_id}/firewall/{ip}/rdns"),
                None,
            )
            .await
    }

    pub async fn get_reverse_dns(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<FirewallReverseDns>, Error> {
        self.client
            .send_request(
                Method::GET,
                &format!("/services/{service_id}/firewall/{ip}/rdns"),
                None,
            )
            .await
    }

    pub async fn upsert_reverse_dns(
        &self,
        service_id: &str,
        ip: &str,
        data: FirewallReverseDns,
    ) -> Result<ApiResponse<()>, Error> {
        let body = serde_json::to_value(&data).ok();
        self.client
            .send_request(
                Method::PUT,
                &format!("/services/{service_id}/firewall/{ip}/rdns"),
                body,
            )
            .await
    }

    pub async fn delete_rule(
        &self,
        service_id: &str,
        ip: &str,
        rule_id: &str,
    ) -> Result<ApiResponse<()>, Error> {
        self.client
            .send_request(
                Method::DELETE,
                &format!("/services/{service_id}/firewall/{ip}/rules/{rule_id}"),
                None,
            )
            .await
    }

    pub async fn get_rules(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<Vec<FirewallRule>>, Error> {
        self.client
            .send_request(
                Method::GET,
                &format!("/services/{service_id}/firewall/{ip}/rules"),
                None,
            )
            .await
    }

    pub async fn get_statistics(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<Vec<FirewallStatistics>>, Error> {
        self.client
            .send_request(
                Method::GET,
                &format!("/services/{service_id}/firewall/{ip}/stats"),
                None,
            )
            .await
    }
}
