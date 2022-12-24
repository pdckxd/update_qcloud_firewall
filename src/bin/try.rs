use chrono::Utc;
use reqwest::header::{CONTENT_TYPE, HOST};
use update_qcloud_firewall::{
    make_auth_string_all_in_one, response::DescribeFirewallRulesResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret_id = dotenv::var("SECRETID").unwrap();
    let secret_key = dotenv::var("SECRETKEY").unwrap();

    let timestamp = Utc::now().timestamp();
    // let timestamp: i64 = 1669792108;
    // println!("{timestamp}");
    let date = Utc::now().format("%Y-%m-%d").to_string();
    // println!("{date}");
    let service = "lighthouse";

    // println!("{secret_id}, {secret_key}");
    // let payload = r#"
    //     {
    //         "InstanceId": "lhins-3jq1gki4",
    //         "Offset": 0,
    //         "Limit": 100
    //     }"#;
    let payload = r#"{"InstanceId":"lhins-3jq1gki4","Offset":0,"Limit":100}"#;
    let host = "lighthouse.tencentcloudapi.com";
    let content_type = "application/json";
    // let canonical_request_string = make_canonical_request_string(payload);
    // let string_to_sign = make_string_to_sign(&canonical_request_string, timestamp, &date);
    // let signature_string = make_signature_string(&secret_key, &string_to_sign, &date, service);
    // let authorization_string =
    //     make_authorization_string(&secret_id, &date, service, &signature_string);
    let authorization_string = make_auth_string_all_in_one(host, content_type, payload, timestamp, &date, &secret_id, &secret_key, service);

    let client = reqwest::Client::new();
    let res: DescribeFirewallRulesResponse = client
        .post("https://lighthouse.tencentcloudapi.com")
        .header("Authorization", authorization_string)
        .header(CONTENT_TYPE, content_type)
        .header(HOST, host)
        .header("X-TC-Action", "DescribeFirewallRules")
        .header("X-TC-Timestamp", format!("{timestamp}"))
        .header("X-TC-Version", "2020-03-24")
        .header("X-TC-Region", "ap-shanghai")
        .body(payload)
        .send()
        .await?
        .json()
        .await?;

    let output = serde_json::to_string_pretty(&res).unwrap();
    println!("{output}");

    Ok(())
}
