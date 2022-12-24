use std::ffi::{c_char, c_float};

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

