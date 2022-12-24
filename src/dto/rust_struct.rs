use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct IpInfo {
    pub ip: String,
    pub ip_decimal: u32,
    pub country: String,
    pub country_iso: String,
    pub country_eu: bool,
    pub latitude: f32,
    pub longitude: f32,
    pub time_zone: String,
    pub asn: String,
    pub asn_org: String,
    pub user_agent: Option<UserAgent>,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct UserAgent {
    pub product: Option<String>,
    pub comment: String,
    pub version: String,
    pub raw_value: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetBIpInfo {
    pub publicip: String,
}


