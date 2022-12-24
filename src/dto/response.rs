use serde::{Deserialize, Serialize};

// Common Error Structure
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
}

// CreateDeleteFirewallRulesResponse
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeleteFirewallRulesResponseRoot {
    #[serde(rename = "Response")]
    pub response: CreateDeleteFirewallRulesResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeleteFirewallRulesResponse {
    #[serde(rename = "Error")]
    pub error: Option<Error>,
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
}


// DescribeFirewallRulesResponse
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeFirewallRulesResponseRoot {
    #[serde(rename = "Response", skip_serializing_if = "Option::is_none")]
    pub response: Option<DescribeFirewallRulesResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeFirewallRulesResponse {
    #[serde(rename = "TotalCount")]
    pub total_count: Option<i64>,
    #[serde(rename = "FirewallRuleSet")]
    pub firewall_rule_set: Option<Vec<FirewallRuleSet>>,
    #[serde(rename = "FirewallVersion")]
    pub firewall_version: Option<i64>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRuleSet {
    #[serde(rename = "AppType", skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "Port")]
    pub port: Option<String>,
    #[serde(rename = "CidrBlock")]
    pub cidr_block: String,
    #[serde(rename = "Action")]
    pub action: Option<String>,
    #[serde(rename = "FirewallRuleDescription")]
    pub firewall_rule_description: Option<String>,
}

