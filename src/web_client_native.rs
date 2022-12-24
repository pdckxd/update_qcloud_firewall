use std::{
    ffi::{CStr, CString},
    ops::Deref,
    os::raw::{c_char, c_void},
};

use crate::{
    c_struct::{IpConfigNative, UserAgentNative},
    web_client::{Callback, WebClient}, CallbackFirewall,
};

#[no_mangle]
pub extern "C" fn rust_version() -> *const c_char {
    CString::new(format!("{:?}", reqwest::Version::HTTP_10))
        .unwrap()
        .into_raw()
}

/// Create WebClient. For C, it creates a WebClient struct pointer
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn create_webapi_client(
    tmp_file_path: *const c_char,
    instance_id: *const c_char,
    token_id: *const c_char,
    token_key: *const c_char,
) -> *mut WebClient {
    let c_str_tmp_file_path = unsafe { CStr::from_ptr(tmp_file_path) };
    let str_slice_tmp_file_path = c_str_tmp_file_path.to_str().unwrap();
    let c_str_instance_id = unsafe { CStr::from_ptr(instance_id) };
    let str_slice_instance_id = c_str_instance_id.to_str().unwrap();
    let c_str_token_id = unsafe { CStr::from_ptr(token_id) };
    let str_slice_token_id = c_str_token_id.to_str().unwrap();
    let c_str_token_key = unsafe { CStr::from_ptr(token_key) };
    let str_slice_token_key = c_str_token_key.to_str().unwrap();
    Box::into_raw(Box::new(WebClient::new(
        str_slice_tmp_file_path,
        str_slice_instance_id,
        str_slice_token_id,
        str_slice_token_key,
    )))
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
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    assert!(!s.is_null());
    // all variable returned from into_raw() need to be freed in Rust.
    drop(Box::from_raw(s));
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

#[allow(non_snake_case)]
#[repr(C)]
pub struct FirewallCallback {
    owner: *mut c_void,
    onResult: extern "C" fn(owner: *mut c_void, arg: *const c_char),
    onError: extern "C" fn(owner: *mut c_void, arg: *const c_char),
}

impl Copy for FirewallCallback {}

impl Clone for FirewallCallback {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe impl Send for FirewallCallback {}

impl Deref for FirewallCallback {
    type Target = FirewallCallback;

    fn deref(&self) -> &FirewallCallback {
        self
    }
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

// /// call qcloud webapi to check if current public ip is allow from firewall
// pub extern "C" fn check_ip_changed(token: *const c_char, loc: *const c_char) -> std::ffi::c_int {
//     1
// }

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
pub unsafe extern "C" fn recreate_firewall_policy(
    client: *mut WebClient,
    payload: *const c_char,
    outer_listener: FirewallCallback,
) {
    assert!(!client.is_null());
    
    let c_str_payload = unsafe { CStr::from_ptr(payload) };
    let str_slice_payload = c_str_payload.to_str().unwrap();

    let local_client = client.as_ref().unwrap();
    let cb = CallbackFirewall {
        result: Box::new(move |result| {
            let message = CString::new(result).unwrap();
            // we don't want to use into_raw to leak the memory of message into C. In this way
            // (as_ptr), rust will finally release CString memory. Otherwise we have to call
            // free_string(xx) from C code to manually release rust CString
            let ptr = message.as_ptr();
            (outer_listener.onResult)(outer_listener.owner, ptr);
        }),
        error: Box::new(move |error| {
            let err_msg = CString::new(error).unwrap();
            let ptr = err_msg.as_ptr();
            (outer_listener.onError)(outer_listener.owner, ptr);
        }),
    };
    let callback = Box::new(cb);
    local_client.recreateFirewallPolicy(str_slice_payload, callback);
}

// #[cfg(test)]
// mod tests {
//     // use super::*;
//
//     #[test]
//     fn test_rust_native_reqwest_get() {
//         tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .unwrap()
//             .block_on(async move {
//                 let body = reqwest::get("http://ifconfig.co/json")
//                     .await
//                     .unwrap()
//                     .text()
//                     .await;
//
//                 println!("body = {:?}", body);
//             });
//     }
// }
