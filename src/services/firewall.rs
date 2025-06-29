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
use crate::VpsAction;

pub struct FirewallApiService {
    client: Client,
    base_url: String,
    access_token: String,
}

impl FirewallApiService {
    pub fn new(client: Client, base_url: String, access_token: String) -> Self {
        Self {
            client,
            base_url,
            access_token,
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
            .bearer_auth(self.access_token.clone())
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
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response
            .json::<ApiResponse<AttackNotificationSettings>>()
            .await
    }

    pub async fn update_attack_notification_settings(
        &self,
        service_id: &str,
        ip: &str,
        settings: AttackNotificationSettings,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/attack-notification",
            self.base_url, service_id, ip
        );

        let mut body = HashMap::new();
        body.insert(
            "emailNotification",
            serde_json::to_value(settings.email_notification).unwrap(),
        );
        if let Some(url_str) = settings.discord_webhook_url {
            body.insert("discordWebhookURL", serde_json::to_value(url_str).unwrap());
        }

        let response = self
            .client
            .post(&url)
            .bearer_auth(self.access_token.clone())
            .json(&body)
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
            "{}/services/{}/firewall/{}/reverse-dns",
            self.base_url, service_id, ip
        );
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response.json::<ApiResponse<FirewallReverseDns>>().await
    }

    pub async fn update_reverse_dns(
        &self,
        service_id: &str,
        ip: &str,
        data: FirewallReverseDns,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/reverse-dns",
            self.base_url, service_id, ip
        );

        let mut body = HashMap::new();
        body.insert("rdns", data.rdns);

        let response = self
            .client
            .post(&url)
            .bearer_auth(self.access_token.clone())
            .json(&body)
            .send()
            .await?;

        response.json::<ApiResponse<()>>().await
    }

    pub async fn delete_rule(
        &self,
        service_id: &str,
        ip: &str,
        rule_id: u32,
    ) -> Result<ApiResponse<()>, reqwest::Error> {
        let url = format!(
            "{}/services/{}/firewall/{}/rules/{}",
            self.base_url, service_id, ip, rule_id
        );
        let response = self
            .client
            .delete(&url)
            .bearer_auth(self.access_token.clone())
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
            .bearer_auth(self.access_token.clone())
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
            .bearer_auth(self.access_token.clone())
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
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        response
            .json::<ApiResponse<Vec<FirewallStatistics>>>()
            .await
    }
}
