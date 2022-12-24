use std::{error, fmt};

use crate::{
    make_auth_string_all_in_one, request::CreateDeleteFirewallRulesRequest,
    response::FirewallRuleSet,
};
use chrono::Utc;
use reqwest::header::{CONTENT_TYPE, HOST};

pub struct QCloudWebClient {
    host: String,
    content_type: String,
    instance_id: String,
    secret_id: String,
    secret_key: String,
    service: String,
}

#[derive(Debug, Clone)]
pub struct QCloudError(pub String);

impl error::Error for QCloudError {}

impl fmt::Display for QCloudError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl QCloudWebClient {
    pub fn new(
        host: String,
        content_type: String,
        instance_id: String,
        secret_id: String,
        secret_key: String,
        service: String,
    ) -> Self {
        Self {
            host,
            content_type,
            instance_id,
            secret_id,
            secret_key,
            service,
        }
    }

    /// Query all firewall rules
    ///
    /// Set VERBOSE=1 to print raw response body
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn query_all_firewall_rules(
        &self,
    ) -> Result<Vec<FirewallRuleSet>, Box<dyn std::error::Error>> {
        let payload = format!(
            "{{\"InstanceId\":\"{}\",\"Offset\":0,\"Limit\":100}}",
            self.instance_id
        );
        let timestamp = Utc::now().timestamp();
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let auth_string = make_auth_string_all_in_one(
            &self.host,
            &self.content_type,
            &payload,
            timestamp,
            &date,
            &self.secret_id,
            &self.secret_key,
            &self.service,
        );

        let host = "lighthouse.tencentcloudapi.com";
        let client = reqwest::Client::new();
        // let res: crate::response::DescribeFirewallRulesResponseRoot = client
        let res = client
            .post(format!("https://{host}"))
            .header("Authorization", auth_string)
            .header(CONTENT_TYPE, &self.content_type)
            .header(HOST, host)
            .header("X-TC-Action", "DescribeFirewallRules")
            .header("X-TC-Timestamp", format!("{timestamp}"))
            .header("X-TC-Version", "2020-03-24")
            .header("X-TC-Region", "ap-shanghai")
            .body(payload)
            .send()
            .await?
            // .json()
            .text()
            .await?;

        if dotenv::var("VERBOSE").unwrap_or_default() == "1" {
            println!("{res}");
        }

        let response: crate::response::DescribeFirewallRulesResponseRoot =
            serde_json::from_str(&res).unwrap();

        let mut result = Vec::new();

        if let Some(resp) = response.response {
            if let Some(e) = resp.error {
                return Err(Box::new(QCloudError(e.message)));
            }
            result = resp.firewall_rule_set.unwrap();
        }

        Ok(result)
    }

    /// Query firewall rules by given description
    ///
    /// Set VERBOSE=1 to print raw response body
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn query_firewall_rules_by_description(
        &self,
        desc_list: &[String],
    ) -> Result<Vec<FirewallRuleSet>, Box<dyn std::error::Error>> {
        let res = self.query_all_firewall_rules().await?;
        // let mut result = Vec::new();
        let result = res
            .iter()
            .filter_map(|rule| {
                rule.firewall_rule_description
                    .as_ref()
                    .filter(|desp| desc_list.contains(desp)).map(|_item|{
                        let mut rule_clone = rule.clone();
                        rule_clone.app_type = None;
                        rule_clone
                    })
            })
            .collect::<Vec<FirewallRuleSet>>();

        // for rule in res.iter() {
        //     if let Some(desc) = &rule.firewall_rule_description {
        //         if desc_list.contains(desc) {
        //             let mut rule_clone = rule.clone();
        //             rule_clone.app_type = None;
        //             result.push(rule_clone);
        //         }
        //     }
        // }

        Ok(result)
    }

    /// Create firewall rules
    ///
    /// Set VERBOSE=1 to print raw response body
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn qcloud_delete_firewall_rules(
        &self,
        payload: &CreateDeleteFirewallRulesRequest,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let payload_new = payload.clone();
        // for rule in payload_new.firewall_rules.iter_mut() {
        //     // rule.cidr_block = None;
        // }
        let payload_str = serde_json::to_string(&payload_new).unwrap();
        let timestamp = Utc::now().timestamp();
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let auth_string = make_auth_string_all_in_one(
            &self.host,
            &self.content_type,
            &payload_str,
            timestamp,
            &date,
            &self.secret_id,
            &self.secret_key,
            &self.service,
        );

        let host = "lighthouse.tencentcloudapi.com";
        let client = reqwest::Client::new();
        // let res: crate::response::CreateDeleteFirewallRulesResponseRoot = client
        let res = client
            .post(format!("https://{host}"))
            .header("Authorization", auth_string)
            .header(CONTENT_TYPE, &self.content_type)
            .header(HOST, host)
            .header("X-TC-Action", "DeleteFirewallRules")
            .header("X-TC-Timestamp", format!("{timestamp}"))
            .header("X-TC-Version", "2020-03-24")
            .header("X-TC-Region", "ap-shanghai")
            .body(payload_str)
            .send()
            .await?
            // .json()
            .text()
            .await?;

        if dotenv::var("VERBOSE").unwrap_or_default() == "1" {
            println!("{res}");
        }

        let response: crate::response::CreateDeleteFirewallRulesResponseRoot =
            serde_json::from_str(&res).unwrap();

        if let Some(e) = response.response.error {
            // if error is caused FirewallRulesNotFound, we don't need to care about
            if e.code != "ResourceNotFound.FirewallRulesNotFound" {
                return Err(Box::new(QCloudError(e.message)));
            }
        }

        Ok(true)
    }

    /// Create firewall rules.
    ///
    /// Set VERBOSE=1 to print raw response body
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn qcloud_create_firewall_rules(
        &self,
        payload: &CreateDeleteFirewallRulesRequest,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let payload_new = payload.clone();
        let payload_str = serde_json::to_string(&payload_new).unwrap();
        let timestamp = Utc::now().timestamp();
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let auth_string = make_auth_string_all_in_one(
            &self.host,
            &self.content_type,
            &payload_str,
            timestamp,
            &date,
            &self.secret_id,
            &self.secret_key,
            &self.service,
        );

        let host = "lighthouse.tencentcloudapi.com";
        let client = reqwest::Client::new();
        // let res: crate::response::CreateDeleteFirewallRulesResponseRoot = client
        let res = client
            .post(format!("https://{host}"))
            .header("Authorization", auth_string)
            .header(CONTENT_TYPE, &self.content_type)
            .header(HOST, host)
            .header("X-TC-Action", "CreateFirewallRules")
            .header("X-TC-Timestamp", format!("{timestamp}"))
            .header("X-TC-Version", "2020-03-24")
            .header("X-TC-Region", "ap-shanghai")
            .body(payload_str)
            .send()
            .await?
            // .json()
            .text()
            .await?;

        if dotenv::var("VERBOSE").unwrap_or_default() == "1" {
            println!("{res}");
        }

        let response: crate::response::CreateDeleteFirewallRulesResponseRoot =
            serde_json::from_str(&res).unwrap();

        if let Some(e) = response.response.error {
            return Err(Box::new(QCloudError(e.message)));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    // use crate::IPHONE11_PAYLOAD_TPL;

    use tokio::{fs::File, io::AsyncReadExt};

    #[allow(unused_imports)]
    use super::*;

    #[tokio::test]
    async fn test_qcloud_query_firewall_rules_by_description_not_exists() {
        let secret_id = dotenv::var("SECRETID").expect("Please specific SECRETID variable in .env file. You can get secret_id from qcloud web portal.");
        let secret_key = dotenv::var("SECRETKEY").expect("Please specific SECRETKEY variable in .env file. You can get secret_key from qcloud web portal.");
        // let secret_key = "123";

        let qcloud_webclient = QCloudWebClient::new(
            "lighthouse.tencentcloudapi.com".to_string(),
            "application/json".to_string(),
            "lhins-3jq1gki4".to_string(),
            secret_id.to_string(),
            secret_key.to_string(),
            "lighthouse".to_string(),
        );
        let result = qcloud_webclient
            .query_firewall_rules_by_description(&["not exist firewall rule name".to_owned()])
            .await
            .unwrap();

        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_qcloud_create_firewall_rules() {
        let secret_id = dotenv::var("SECRETID").expect("Please specific SECRETID variable in .env file. You can get secret_id from qcloud web portal.");
        let secret_key = dotenv::var("SECRETKEY").expect("Please specific SECRETKEY variable in .env file. You can get secret_key from qcloud web portal.");

        let qcloud_webclient = QCloudWebClient::new(
            "lighthouse.tencentcloudapi.com".to_string(),
            "application/json".to_string(),
            "lhins-3jq1gki4".to_string(),
            secret_id.to_string(),
            secret_key.to_string(),
            "lighthouse".to_string(),
        );

        // let request_payload = IPHONE11_PAYLOAD_TPL;
        let mut f = File::open("iphone_payload.json").await.unwrap();
        // let request_payload = IPHONE11_PAYLOAD_TPL;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await.unwrap();
        let request_payload = String::from_utf8(buf).unwrap();
        let request: CreateDeleteFirewallRulesRequest =
            serde_json::from_str(&request_payload).unwrap();
        let result = qcloud_webclient
            .qcloud_create_firewall_rules(&request)
            .await
            .unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_qcloud_delete_firewall_rules() {
        let secret_id = dotenv::var("SECRETID").expect("Please specific SECRETID variable in .env file. You can get secret_id from qcloud web portal.");
        let secret_key = dotenv::var("SECRETKEY").expect("Please specific SECRETKEY variable in .env file. You can get secret_key from qcloud web portal.");

        let qcloud_webclient = QCloudWebClient::new(
            "lighthouse.tencentcloudapi.com".to_string(),
            "application/json".to_string(),
            "lhins-3jq1gki4".to_string(),
            secret_id.to_string(),
            secret_key.to_string(),
            "lighthouse".to_string(),
        );

        // let request_payload = IPHONE11_PAYLOAD_TPL;
        let mut f = File::open("iphone_payload.json").await.unwrap();
        // let request_payload = IPHONE11_PAYLOAD_TPL;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await.unwrap();
        let request_payload = String::from_utf8(buf).unwrap();
        let request: CreateDeleteFirewallRulesRequest =
            serde_json::from_str(&request_payload).unwrap();
        let result = qcloud_webclient
            .qcloud_delete_firewall_rules(&request)
            .await
            .unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_auth_with_wrong_authid_or_authkey_should_fail() {
        let secret_key = "123";
        let secret_id = "123";

        let qcloud_webclient = QCloudWebClient::new(
            "lighthouse.tencentcloudapi.com".to_string(),
            "application/json".to_string(),
            "lhins-3jq1gki4".to_string(),
            secret_id.to_string(),
            secret_key.to_string(),
            "lighthouse".to_string(),
        );
        let result = qcloud_webclient
            .query_firewall_rules_by_description(&["not exist firewall rule name".to_owned()])
            .await;

        assert!(result.is_err());

        if let Err(msg) = result {
            assert_eq!(
                format!("{}", msg),
                "The SecretId is not found, please ensure that your SecretId is correct."
            );
        }
    }
}
