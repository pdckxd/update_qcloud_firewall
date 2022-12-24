use std::ffi::{c_char, CStr, CString};
use std::path::Path;

use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::request::{CreateDeleteFirewallRulesRequest, FirewallRule};
use crate::rust_struct::{IpInfo, SetBIpInfo};
use crate::{QCloudError, QCloudWebClient};

//Lazy static
lazy_static! {
    //runtime with threaded pool
    static ref RUN_TIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    // HTTP client to share
    // static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true).build().unwrap();
}

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Safety
///
/// .
pub unsafe fn cchar_to_string(c_str: *mut c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(c_str) };
    cstr.to_str().unwrap().to_string()
}
// const IP_TXT: &str = "ip.txt";
// const IPHONE_TPL: &str = "iphone.tpl";

/// Convert reference of str to mutable c_char pointer.
/// Warning: Must release String from Rust otherwise it will lead to memory leak.
///
/// # Panics
///
/// Panics if .
pub fn str_to_c_char_ptr(p_str: &str) -> *mut c_char {
    let cstr = CString::new(p_str).unwrap();
    cstr.into_raw()
}

//Local callback for loading
pub struct Callback {
    pub result: Box<dyn FnMut(IpInfo)>,
    pub error: Box<dyn FnMut(String)>,
}

//Callback trait
#[allow(non_snake_case)]
pub trait WebApiCallback {
    /// Be called if there is no error.
    fn onLoad(&mut self, ip_info: IpInfo);

    /// Be called when error occurs.
    fn onError(&mut self, s: &str);
}

/// Implement WebApiCallback trait for Callback structure
#[allow(non_snake_case)]
impl WebApiCallback for Callback {
    fn onLoad(&mut self, ip_info: IpInfo) {
        (self.result)(ip_info);
    }

    fn onError(&mut self, s: &str) {
        (self.error)(s.to_string());
    }
}

unsafe impl Send for Callback {}

//Local callback for loading
pub struct CallbackFirewall {
    pub result: Box<dyn FnMut(String)>,
    pub error: Box<dyn FnMut(String)>,
}

//Callback trait
#[allow(non_snake_case)]
pub trait WebApiCallbackFirewall {
    /// Be called if there is no error.
    fn onLoad(&mut self, s: &str);

    /// Be called when error occurs.
    fn onError(&mut self, s: &str);
}

/// Implement WebApiCallback trait for Callback structure
#[allow(non_snake_case)]
impl WebApiCallbackFirewall for CallbackFirewall {
    fn onLoad(&mut self, s: &str) {
        (self.result)(s.to_owned());
    }

    fn onError(&mut self, s: &str) {
        (self.error)(s.to_string());
    }
}

unsafe impl Send for CallbackFirewall {}

#[repr(C)]
pub enum PayloadType {
    IPHONE,
    PDRD,
    LCRD,
}

#[repr(C)]
pub struct WebClient {
    tmp_file_path: *mut c_char,
    instance_id: *mut c_char,
    token_id: *mut c_char,
    token_key: *mut c_char,
}

impl Drop for WebClient {
    fn drop(&mut self) {
        unsafe {
            drop(CString::from_raw(self.tmp_file_path));
            drop(CString::from_raw(self.instance_id));
            drop(CString::from_raw(self.token_id));
            drop(CString::from_raw(self.token_key));
        }
    }
}

#[allow(non_snake_case)]
impl WebClient {
    pub fn new(
        tmp_file_path: &str,
        instance_id: &str,
        token_id: &str,
        token_key: &str,
    ) -> WebClient {
        let ptr_tmp_file_path = str_to_c_char_ptr(tmp_file_path);
        let ptr_instance_id = str_to_c_char_ptr(instance_id);
        let ptr_token_id = str_to_c_char_ptr(token_id);
        let ptr_token_key = str_to_c_char_ptr(token_key);
        WebClient {
            tmp_file_path: ptr_tmp_file_path,
            instance_id: ptr_instance_id,
            token_id: ptr_token_id,
            token_key: ptr_token_key,
        }
    }

    /// Wrap tokio async function to call reqwest async function and give back the result or error
    /// through callback function
    pub fn getIpConfig(&self, mut callback: Box<dyn WebApiCallback + Send>) {
        (*RUN_TIME).block_on(async move {
            let c_str = unsafe { CStr::from_ptr(self.tmp_file_path) };
            let tmp_file_path = c_str.to_str().unwrap().to_string();
            let ip_tools = IpTools::new(tmp_file_path);
            let res = ip_tools.get_ip_config().await;
            match res {
                Ok(root) => {
                    //print response
                    //println!("Response: {:#?}", root.results);
                    callback.onLoad(root);
                }
                Err(err) => {
                    let error = format!("Failed to get ip config. Err: {}", err);
                    println!("Error: {}", error);
                    callback.onError(error.as_str())
                }
            }
        });
    }

    pub fn recreateFirewallPolicy(
        &self,
        request_payload: &str,
        mut callback: Box<dyn WebApiCallbackFirewall + Send>,
    ) {
        (*RUN_TIME).block_on(async move {
            let tmp_file_path = unsafe { cchar_to_string(self.tmp_file_path) };
            let instance_id = unsafe { cchar_to_string(self.instance_id) };
            let token_id = unsafe { cchar_to_string(self.token_id) };
            let token_key = unsafe { cchar_to_string(self.token_key) };

            let ip_tools = IpTools::new(tmp_file_path);
            let ip = ip_tools.get_china_ip_address().await.unwrap();
            let res = ip_tools.check_ip_changed(&ip).await.unwrap();
            // if ip not changed, exit immediately
            if !res {
                return;
            }
            let qcloud_tool = QCloudTool::new(Some(token_id), Some(token_key));
            // let request_payload: String = match self.payload_type {
            //     PayloadType::IPHONE => IPHONE11_PAYLOAD_TPL.to_string(),
            //     PayloadType::PDRD => todo!(),
            //     PayloadType::LCRD => todo!(),
            // };

            // we remove existing firewall rules.
            let request: CreateDeleteFirewallRulesRequest =
                serde_json::from_str(request_payload).unwrap();
            let res = qcloud_tool
                .remove_firewall_rules(&instance_id, &request)
                .await;
            match res {
                Ok(_root) => {
                    //print response
                    //println!("Response: {:#?}", root.results);
                    callback.onLoad("Sucessfully remove firewall policy!");
                }
                Err(err) => {
                    let error = format!("Failed to remove firewall policy. Err: {}", err);
                    // println!("Error: {}", error);
                    callback.onError(error.as_str());
                    return;
                }
            }

            // we create new firewall rules with new public ip in cidr field
            let res = qcloud_tool
                .create_firewall_rules(&instance_id, &request, &ip)
                .await;
            match res {
                Ok(_root) => {
                    //print response
                    //println!("Response: {:#?}", root.results);
                    callback.onLoad("Sucessfully create firewall policy!");
                    // if ip changes, we need to recreate firewall and save the ip into temp file
                    ip_tools.save_ip_into_file(&ip).await.unwrap();
                }
                Err(err) => {
                    let error = format!("Failed to create firewall policy. Err: {}", err);
                    // println!("Error: {}", error);
                    callback.onError(error.as_str())
                }
            }
        });
    }
}

pub struct IpTools {
    tmp_file_path: String,
}

impl IpTools {
    pub fn new(tmp_file_path: String) -> Self {
        Self { tmp_file_path }
    }

    /// Actual calling reqwest::get and convert json response body to a struct
    pub async fn get_ip_config(&self) -> Result<IpInfo, Box<dyn std::error::Error>> {
        let ip_config: IpInfo = HTTP_CLIENT
            .get("http://ifconfig.co/json")
            .send()
            .await?
            .json()
            .await?;
        Ok(ip_config)
    }

    pub async fn get_china_ip_address(&self) -> Result<String, Box<dyn std::error::Error>> {
        let ip_config: SetBIpInfo = HTTP_CLIENT
            .get("https://setb.cn/ip.json")
            .send()
            .await?
            .json()
            .await?;
        Ok(ip_config.publicip)
    }

    pub async fn check_ip_changed(
        &self,
        public_ip: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if !Path::new(&self.tmp_file_path).exists() {
            // file not exist
            return Ok(true);
        }
        let mut f = File::open(&self.tmp_file_path).await.unwrap();
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer).await?;
        let ip_addr = String::from_utf8_lossy(&buffer);
        if public_ip.trim() == ip_addr.trim() {
            // ip not change
            Ok(false)
        } else {
            // ip changed
            Ok(true)
        }
    }

    pub async fn save_ip_into_file(
        &self,
        public_ip: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if Path::new(&self.tmp_file_path).exists() {
            fs::remove_file(&self.tmp_file_path).await.unwrap_or(());
        }
        let mut file = File::create(&self.tmp_file_path).await?;

        file.write_all(public_ip.trim().as_bytes()).await?;
        Ok(true)
    }
}

pub struct QCloudTool {
    secret_id: String,
    secret_key: String,
}

impl QCloudTool {
    pub fn new(secret_id: Option<String>, secret_key: Option<String>) -> Self {
        if secret_id.is_none() | secret_key.is_none() {
            return Self {
                secret_id: dotenv::var("SECRETID").unwrap(),
                secret_key: dotenv::var("SECRETKEY").unwrap(),
            };
        }
        Self {
            secret_id: secret_id.unwrap(),
            secret_key: secret_key.unwrap(),
        }
    }
    pub async fn remove_firewall_rules(
        &self,
        instance_id: &str,
        tpl: &CreateDeleteFirewallRulesRequest,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // let mut desc_list = vec![];
        let desc_list = tpl
            .firewall_rules
            .iter()
            .filter_map(|item| item.firewall_rule_description.clone())
            .collect::<Vec<String>>();
        // for rule in tpl.firewall_rules.iter() {
        //     if let Some(desc) = &rule.firewall_rule_description {
        //         desc_list.push(desc.clone());
        //     }
        // }
        // println!("{}, {}", self.secret_id, self.secret_key);

        let qcloud_webclient = QCloudWebClient::new(
            "lighthouse.tencentcloudapi.com".to_string(),
            "application/json".to_string(),
            instance_id.to_string(),
            self.secret_id.to_string(),
            self.secret_key.to_string(),
            "lighthouse".to_string(),
        );
        let result = qcloud_webclient
            .query_firewall_rules_by_description(&desc_list)
            .await?;

        // let mut firewall_rules_to_delete = Vec::new();
        let firewall_rules_to_delete: Vec<FirewallRule> = result.iter().map(|rule| {
            FirewallRule {
                protocol: Some(rule.protocol.clone()),
                port: rule.port.clone(),
                cidr_block: Some(rule.cidr_block.clone()),
                action: rule.action.clone(),
                firewall_rule_description: rule.firewall_rule_description.clone(),
            }
        }).collect();

        // for rule in result.iter() {
        //     firewall_rules_to_delete.push(FirewallRule {
        //         protocol: Some(rule.protocol.clone()),
        //         port: rule.port.clone(),
        //         cidr_block: Some(rule.cidr_block.clone()),
        //         action: rule.action.clone(),
        //         firewall_rule_description: rule.firewall_rule_description.clone(),
        //     });
        // }
        //
        if firewall_rules_to_delete.is_empty() {
            println!("Not found existing firewall rules to be deleted");
            return Ok(true);
        }

        let request_payload = CreateDeleteFirewallRulesRequest {
            instance_id: instance_id.to_string(),
            firewall_rules: firewall_rules_to_delete,
        };

        let _result = qcloud_webclient
            .qcloud_delete_firewall_rules(&request_payload)
            .await?;

        println!(
            "Sucessfully delete {} rules",
            request_payload.firewall_rules.len()
        );

        Ok(true)
    }

    pub async fn create_firewall_rules(
        &self,
        instance_id: &str,
        tpl: &CreateDeleteFirewallRulesRequest,
        ip_address: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut tpl_clone = tpl.clone();
        // let mut desc_list = vec![];
        let desc_list = tpl.firewall_rules.iter().filter_map(|rule| {
            rule.firewall_rule_description.clone()
        }).collect::<Vec<String>>();
        // for rule in tpl.firewall_rules.iter() {
        //     if let Some(desc) = &rule.firewall_rule_description {
        //         desc_list.push(desc.clone());
        //     }
        // }
        // println!("{}, {}", self.secret_id, self.secret_key);

        let qcloud_webclient = QCloudWebClient::new(
            "lighthouse.tencentcloudapi.com".to_string(),
            "application/json".to_string(),
            instance_id.to_string(),
            self.secret_id.to_string(),
            self.secret_key.to_string(),
            "lighthouse".to_string(),
        );
        let result = qcloud_webclient
            .query_firewall_rules_by_description(&desc_list)
            .await?;

        if !result.is_empty() {
            return Err(Box::new(QCloudError(
                "Rule(s) exist(s) in firewall. Please delete at first!".to_owned(),
            )));
        }

        // Change cidr with given public ip address
        for rule in tpl_clone.firewall_rules.iter_mut() {
            rule.cidr_block = Some(ip_address.to_owned());
        }

        let request_payload = CreateDeleteFirewallRulesRequest {
            instance_id: instance_id.to_string(),
            firewall_rules: tpl_clone.firewall_rules,
        };

        let _result = qcloud_webclient
            .qcloud_create_firewall_rules(&request_payload)
            .await?;

        println!(
            "Sucessfully create {} rules",
            request_payload.firewall_rules.len()
        );

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    // use crate::IPHONE11_PAYLOAD_TPL;

    use super::*;

    #[tokio::test]
    async fn test_save_ip_into_file() {
        let tmp_file_path = std::env::temp_dir()
            .join("update_qcloud_firewall_ip.txt")
            .as_path()
            .display()
            .to_string();
        let ip_tools = IpTools::new(tmp_file_path);
        assert!(ip_tools.save_ip_into_file("127.0.0.1").await.unwrap());
    }

    #[tokio::test]
    async fn test_get_china_ip_address() {
        let tmp_file_path = std::env::temp_dir()
            .join("update_qcloud_firewall_ip.txt")
            .as_path()
            .display()
            .to_string();
        let ip_tools = IpTools::new(tmp_file_path);
        let ip = ip_tools.get_china_ip_address().await.unwrap();
        println!("{}", ip);
        assert!(!ip.is_empty());
    }

    #[tokio::test]
    async fn test_check_ip_changed() {
        let tmp_file_path = std::env::temp_dir()
            .join("update_qcloud_firewall_ip.txt")
            .as_path()
            .display()
            .to_string();
        let ip_tools = IpTools::new(tmp_file_path);
        ip_tools.save_ip_into_file("127.0.0.1").await.unwrap();
        let ip = ip_tools.get_china_ip_address().await.unwrap();
        assert!(ip_tools.check_ip_changed(&ip).await.unwrap());
    }

    #[tokio::test]
    async fn test_check_ip_changed_not_ip_txt() {
        let tmp_file_path = std::env::temp_dir()
            .join("update_qcloud_firewall_ip.txt")
            .as_path()
            .display()
            .to_string();
        let ip_tools = IpTools::new(tmp_file_path.clone());
        fs::remove_file(&tmp_file_path).await.unwrap_or(());
        let ip = ip_tools.get_china_ip_address().await.unwrap();
        assert!(ip_tools.check_ip_changed(&ip).await.unwrap());
    }

    #[tokio::test]
    async fn test_check_ip_not_changed() {
        let tmp_file_path = std::env::temp_dir()
            .join("update_qcloud_firewall_ip.txt")
            .as_path()
            .display()
            .to_string();
        let ip_tools = IpTools::new(tmp_file_path);
        let ip = ip_tools.get_china_ip_address().await.unwrap();
        ip_tools.save_ip_into_file(&ip).await.unwrap();
        assert!(!ip_tools.check_ip_changed(&ip).await.unwrap());
    }

    #[tokio::test]
    async fn test_create_firewall_rules() {
        let mut f = File::open("iphone_payload.json").await.unwrap();
        // let request_payload = IPHONE11_PAYLOAD_TPL;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await.unwrap();
        let request_payload = String::from_utf8(buf).unwrap();
        let request: CreateDeleteFirewallRulesRequest =
            serde_json::from_str(&request_payload).unwrap();
        let qcloud_tool = QCloudTool::new(None, None);
        qcloud_tool
            .create_firewall_rules("lhins-3jq1gki4", &request, "127.0.0.1")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_remove_firewall_rules() {
        let mut f = File::open("iphone_payload.json").await.unwrap();
        // let request_payload = IPHONE11_PAYLOAD_TPL;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await.unwrap();
        let request_payload = String::from_utf8(buf).unwrap();
        let request: CreateDeleteFirewallRulesRequest =
            serde_json::from_str(&request_payload).unwrap();
        let qcloud_tool = QCloudTool::new(None, None);
        qcloud_tool
            .remove_firewall_rules("lhins-3jq1gki4", &request)
            .await
            .unwrap();
    }

    // #[test]
    // fn test_rust_native_reqwest_get() {
    //     tokio::runtime::Builder::new_multi_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap()
    //         .block_on(async move {
    //             let body = reqwest::get("http://ifconfig.co/json")
    //                 .await
    //                 .unwrap()
    //                 .text()
    //                 .await;
    //
    //             println!("body = {:?}", body);
    //         });
    // }
}
