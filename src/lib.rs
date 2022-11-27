#[macro_use]
extern crate lazy_static;

// pub mod webclient;
// use std::{ffi::CStr, os::raw::c_char};
//
// #[no_mangle]
// pub extern "C" fn add(left: u32, right: u32) -> u32 {
//     left + right
// }

/// .
///
/// # Safety
///
/// .
// pub unsafe extern "C" fn update_firewall_policy(token_ptr: *const c_char, loc_ptr: *const c_char) -> u8 {
//     let token =   CStr::from_ptr(token_ptr) ;
//     let loc =  CStr::from_ptr(loc_ptr);
//
//     let ip_info = IpInfo();
//
//     1
// }
use serde::{Deserialize, Serialize};
use std::{
    ffi::CString,
    ops::Deref,
    os::raw::{c_char, c_float, c_void},
};

//Lazy static
lazy_static! {
    //runtime with threaded pool
    static ref RUN_TIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    // HTTP client to share
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

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

//Callback trait
#[allow(non_snake_case)]
pub trait WebApiCallback {
    /// Be called if there is no error.
    fn onLoad(&mut self, ip_info: IpInfo);

    /// Be called when error occurs.
    fn onError(&mut self, s: &str);
}

#[repr(C)]
pub struct WebClient();

#[allow(non_snake_case)]
impl WebClient {
    pub fn new() -> WebClient {
        WebClient()
    }

    /// Wrap tokio async function to call reqwest async function and give back the result or error
    /// through callback function
    pub fn getIpConfig(&self, mut callback: Box<dyn WebApiCallback + Send>) {
        (*RUN_TIME).block_on(async move {
            let res = get_ip_config().await;
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
}

impl Default for WebClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Actual calling reqwest::get and convert json response body to a struct
pub async fn get_ip_config() -> Result<IpInfo, Box<dyn std::error::Error>> {
    let ip_config: IpInfo = HTTP_CLIENT
        .get("http://ifconfig.co/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(ip_config)
}

/// Create WebClient. For C, it creates a WebClient struct pointer
#[no_mangle]
pub extern "C" fn create_webapi_client() -> *mut WebClient {
    Box::into_raw(Box::new(WebClient::new()))
}

/// .
///
/// # Panics
///
/// Panics if client is null.
///
/// # Safety
///
/// Release WebClient memory because it is wrapped into Box.
/// For C, Release memory of WebClient struct pointer
#[no_mangle]
pub unsafe extern "C" fn free_swapi_client(client: *mut WebClient) {
    assert!(!client.is_null());
    drop(Box::from_raw(client));
}

#[repr(C)]
pub struct IpConfigNative {
    pub ip: *const c_char,
    pub ip_decimal: u32,
    pub country: *const c_char,
    pub country_iso: *const c_char,
    pub country_eu: u8,
    pub latitude: c_float,
    pub longitude: c_float,
    pub time_zone: *const c_char,
    pub asn: *const c_char,
    pub asn_org: *const c_char,
    pub(crate) user_agent: *mut UserAgentNative,
}

#[repr(C)]
pub struct UserAgentNative {
    pub product: *const c_char,
    pub comment: *const c_char,
    pub version: *const c_char,
    pub raw_value: *const c_char,
}

/// you need reference to owner context to return data
#[allow(non_snake_case)]
#[repr(C)]
pub struct IpConfigCallback {
    owner: *mut c_void,
    onResult: extern "C" fn(owner: *mut c_void, arg: *const IpConfigNative),
    onError: extern "C" fn(owner: *mut c_void, arg: *const c_char),
}

impl Copy for IpConfigCallback {}

impl Clone for IpConfigCallback {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe impl Send for IpConfigCallback {}

impl Deref for IpConfigCallback {
    type Target = IpConfigCallback;

    fn deref(&self) -> &IpConfigCallback {
        self
    }
}

//Local callback for loading
pub struct Callback {
    pub result: Box<dyn FnMut(IpInfo)>,
    pub error: Box<dyn FnMut(String)>,
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

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Safety
///
/// .
#[no_mangle]
pub unsafe extern "C" fn get_ip_config_native(
    client: *mut WebClient,
    outer_listener: IpConfigCallback,
) {
    assert!(!client.is_null());

    let local_client = client.as_ref().unwrap();
    let cb = Callback {
        result: Box::new(move |result| {
            let has_user_agent = result.user_agent.is_some();
            let mut user_agent_native: UserAgentNative = if result.user_agent.is_some() {
                let user_agent = result.user_agent.unwrap();
                UserAgentNative {
                    product: if user_agent.product.is_some() {
                        CString::new(user_agent.product.unwrap())
                            .unwrap()
                            .into_raw()
                    } else {
                        std::ptr::null()
                    },
                    comment: CString::new(user_agent.comment).unwrap().into_raw(),
                    version: CString::new(user_agent.version).unwrap().into_raw(),
                    raw_value: CString::new(user_agent.raw_value).unwrap().into_raw(),
                }
            } else {
                UserAgentNative {
                    product: std::ptr::null(),
                    comment: std::ptr::null(),
                    version: std::ptr::null(),
                    raw_value: std::ptr::null(),
                }
            };

            let ip_config_native: IpConfigNative = IpConfigNative {
                ip: CString::new(result.ip).unwrap().into_raw(),
                ip_decimal: result.ip_decimal,
                country: CString::new(result.country).unwrap().into_raw(),
                country_iso: CString::new(result.country_iso).unwrap().into_raw(),
                country_eu: u8::from(result.country_eu),
                latitude: result.latitude,
                longitude: result.longitude,
                time_zone: CString::new(result.time_zone).unwrap().into_raw(),
                asn: CString::new(result.asn).unwrap().into_raw(),
                asn_org: CString::new(result.asn_org).unwrap().into_raw(),
                user_agent: if has_user_agent {
                    &mut user_agent_native
                } else {
                    std::ptr::null_mut()
                },
            };

            (outer_listener.onResult)(outer_listener.owner, &ip_config_native);
        }),
        error: Box::new(move |error| {
            let error_message = CString::new(error).unwrap().into_raw();
            (outer_listener.onError)(outer_listener.owner, error_message);
        }),
    };
    let callback = Box::new(cb);
    local_client.getIpConfig(callback);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_rust_native_reqwest_get() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let body = reqwest::get("http://ifconfig.co/json")
                    .await
                    .unwrap()
                    .text()
                    .await;

                println!("body = {:?}", body);
            });
    }
}
