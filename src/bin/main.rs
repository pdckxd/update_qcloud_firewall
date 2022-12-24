use std::path::Path;

use clap::{command, Parser};
use tokio::{fs::File, io::AsyncReadExt};
use update_qcloud_firewall::{
    request::CreateDeleteFirewallRulesRequest,
    IpTools, QCloudTool,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    payload_json_file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret_id = dotenv::var("SECRETID").unwrap();
    let secret_key = dotenv::var("SECRETKEY").unwrap();
    let instance_id = dotenv::var("INSTANCEID").unwrap();
    let args = Args::parse();

    let mut payload_file = File::open(args.payload_json_file).await?;
    let mut request_payload = String::new();
    payload_file.read_to_string(&mut request_payload).await?;

    let tmp_dir = std::env::temp_dir();
    let tmp_ip_file = Path::new(&tmp_dir).join("update_qcloud_firewall_ip.txt");
    let iptools = IpTools::new(tmp_ip_file.to_string_lossy().to_string());

    let ip_address = iptools.get_china_ip_address().await?;
    if !(iptools.check_ip_changed(&ip_address).await?) {
        println!("Nothing happened since ip address doesn't change.");
        return Ok(());
    }

    let qcloud_tool = QCloudTool::new(Some(secret_id), Some(secret_key));

    // we remove existing firewall rules.
    let request: CreateDeleteFirewallRulesRequest = serde_json::from_str(&request_payload).unwrap();
    let _res = qcloud_tool
        .remove_firewall_rules(&instance_id, &request)
        .await?;

    // we create new firewall rules with new public ip in cidr field
    let _res = qcloud_tool
        .create_firewall_rules(&instance_id, &request, &ip_address)
        .await?;

    iptools.save_ip_into_file(&ip_address).await?;

    Ok(())
}
