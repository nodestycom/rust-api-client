use crate::models::{
    firewall::{
        AttackNotificationSettings, FirewallAttackLog, FirewallCreateRuleData, FirewallReverseDns,
        FirewallRule, FirewallStatistics,
    },
    ApiResponse,
};
use crate::NodestyApiClient;
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
                &format!("/services/{}/firewall/{}/attack-logs", service_id, ip),
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
                &format!(
                    "/services/{}/firewall/{}/attack-notification",
                    service_id, ip
                ),
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
                &format!(
                    "/services/{}/firewall/{}/attack-notification",
                    service_id, ip
                ),
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
                &format!("/services/{}/firewall/{}/rdns", service_id, ip),
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
                &format!("/services/{}/firewall/{}/rdns", service_id, ip),
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
                &format!("/services/{}/firewall/{}/rdns", service_id, ip),
                body,
            )
            .await
    }

    pub async fn delete_rule(
        &self,
        service_id: &str,
        ip: &str,
        rule_id: u32,
    ) -> Result<ApiResponse<()>, Error> {
        self.client
            .send_request(
                Method::DELETE,
                &format!("/services/{}/firewall/{}/rules/{}", service_id, ip, rule_id),
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
                &format!("/services/{}/firewall/{}/rules", service_id, ip),
                None,
            )
            .await
    }

    pub async fn create_rule(
        &self,
        service_id: &str,
        ip: &str,
        data: FirewallCreateRuleData,
    ) -> Result<ApiResponse<()>, Error> {
        let body = serde_json::to_value(&data).ok();
        self.client
            .send_request(
                Method::POST,
                &format!("/services/{}/firewall/{}/rules", service_id, ip),
                body,
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
                &format!("/services/{}/firewall/{}/stats", service_id, ip),
                None,
            )
            .await
    }
}
