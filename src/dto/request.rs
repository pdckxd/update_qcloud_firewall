use serde::{Deserialize, Serialize};

// CreateFirewallRulesRequest
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeleteFirewallRulesRequest {
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "FirewallRules")]
    pub firewall_rules: Vec<FirewallRule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRule {
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<String>,
    #[serde(rename = "CidrBlock", skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    #[serde(rename = "Action")]
    pub action: Option<String>,
    #[serde(rename = "FirewallRuleDescription")]
    pub firewall_rule_description: Option<String>,
}
