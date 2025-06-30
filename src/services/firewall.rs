use crate::models::{
    firewall::{
        AttackNotificationSettings,
        FirewallAttackLog,
        FirewallCreateRuleData,
        FirewallReverseDns,
        FirewallRule,
        FirewallStatistics,
    },
    ApiResponse,
};
use reqwest::Client;
use std::collections::HashMap;

pub struct FirewallApiService {
    client: Client,
    base_url: String,
}

impl FirewallApiService {
    pub fn new(client: Client, base_url: String) -> Self {
        Self {
            client,
            base_url,
        }
    }

    pub async fn get_attack_logs(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<Vec<FirewallAttackLog>>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/attack-logs",
            self.base_url, service_id, ip
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<Vec<FirewallAttackLog>>>().await
    }

    pub async fn get_attack_notification_settings(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<AttackNotificationSettings>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/attack-notification",
            self.base_url, service_id, ip
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response
            .json::<ApiResponse<AttackNotificationSettings>>()
            .await
    }

    pub async fn set_attack_notification_settings(
        &self,
        service_id: &str,
        ip: &str,
        data: AttackNotificationSettings,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/attack-notification",
            self.base_url, service_id, ip
        );
        let response = self
            .client
            .post(&url)
            .json(&data)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }

    pub async fn get_reverse_dns(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<FirewallReverseDns>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/rdns",
            self.base_url, service_id, ip
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<FirewallReverseDns>>().await
    }

    pub async fn set_reverse_dns(
        &self,
        service_id: &str,
        ip: &str,
        data: FirewallReverseDns,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/rdns",
            self.base_url, service_id, ip
        );
        let response = self
            .client
            .post(&url)
            .json(&data)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }

    pub async fn get_rules(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<Vec<FirewallRule>>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/rules",
            self.base_url, service_id, ip
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<Vec<FirewallRule>>>().await
    }

    pub async fn create_rule(
        &self,
        service_id: &str,
        ip: &str,
        data: FirewallCreateRuleData,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/rules",
            self.base_url, service_id, ip
        );
        let response = self
            .client
            .post(&url)
            .json(&data)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }

    pub async fn get_statistics(
        &self,
        service_id: &str,
        ip: &str,
    ) -> Result<ApiResponse<Vec<FirewallStatistics>>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/stats",
            self.base_url, service_id, ip
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        response.json::<ApiResponse<Vec<FirewallStatistics>>>().await
    }
}