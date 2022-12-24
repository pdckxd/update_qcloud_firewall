use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

#[allow(dead_code)]
type HmacSha256 = Hmac<Sha256>;

fn sha_256_hex(input: &str) -> (Vec<u8>, String) {
    let hash = Sha256::digest(input.as_bytes());
    let mut buf = [0u8; 64];
    let bits = hash.to_vec();
    let hex = base16ct::lower::encode_str(&hash, &mut buf)
        .unwrap()
        .to_string();

    (bits.to_vec(), hex)
}

fn hmac_256(key: &[u8], input: &str) -> (Vec<u8>, String) {
    let mut mac = HmacSha256::new_from_slice(key).unwrap();
    mac.update(input.as_bytes());
    let result = mac.finalize();

    let mut buf = [0u8; 64];
    let bits = result.into_bytes();
    let hex = base16ct::lower::encode_str(&bits, &mut buf)
        .unwrap()
        .to_string();
    (bits.to_vec(), hex)
}

pub fn make_auth_string_all_in_one(
    host: &str,
    content_type: &str,
    payload: &str,
    timestamp: i64,
    date: &str,
    secret_id: &str,
    secret_key: &str,
    service: &str,
) -> String {

    let canonical_request_string = make_canonical_request_string(host, content_type, payload);
    let string_to_sign = make_string_to_sign(&canonical_request_string, timestamp, date, service);
    let signature_string = make_signature_string(secret_key, &string_to_sign, date, service);
    // println!("{signature_string}");
    make_authorization_string(secret_id, date, service, &signature_string)
}

pub fn make_canonical_request_string(host: &str, content_type: &str, payload: &str) -> String {
    // let host = "lighthouse.tencentcloudapi.com";
    let http_request_method = "POST";
    let canonical_uri = "/";
    let canonical_querystring = "";
    let canonical_headers = format!("content-type:{content_type}\nhost:{host}\n");
    let signed_headers = "content-type;host";
    // let payload = r#"
    //     {
    //         "InstanceId": "lhins-3jq1gki4",
    //         "Offset": 0,
    //         "Limit": 100
    //     }"#;
    // let payload =
    //     r#"{"Limit": 1, "Filters": [{"Values": ["\u672a\u547d\u540d"], "Name": "instance-name"}]}"#;
    // println!("{}", sha_256_hex(payload).1);
    let hashed_request_payload = sha_256_hex(payload).1;

    format!(
        r#"{http_request_method}
{canonical_uri}
{canonical_querystring}
{canonical_headers}
{signed_headers}
{hashed_request_payload}"#
    )
}

pub fn make_string_to_sign(canonical_request: &str, timestamp: i64, date: &str, service: &str) -> String {
    let algorithm = "TC3-HMAC-SHA256";

    let credential_scope = format!("{date}/{service}/tc3_request");

    let hashed_canonical_request = sha_256_hex(canonical_request).1;
    format!(
        r#"{algorithm}
{timestamp}
{credential_scope}
{hashed_canonical_request}"#
    )
}

pub fn make_signature_string(
    secret_key: &str,
    string_to_sign: &str,
    date: &str,
    service: &str,
) -> String {
    let secret_date = hmac_256(format!("TC3{secret_key}").as_bytes(), date).0;
    let secret_service = hmac_256(&secret_date, service).0;
    let secret_signing = hmac_256(&secret_service, "tc3_request").0;
    hmac_256(&secret_signing, string_to_sign).1
}

pub fn make_authorization_string(
    secret_id: &str,
    date: &str,
    service: &str,
    signature: &str,
) -> String {
    let algorithm = "TC3-HMAC-SHA256";
    // let service = "lighthouse";
    let credential_scope = format!("{date}/{service}/tc3_request");
    format!(
        r#"{algorithm} Credential={secret_id}/{credential_scope}, SignedHeaders=content-type;host, Signature={signature}"#
    )
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_sha_256_hex() {
        let payload = r#"{"Limit": 1, "Filters": [{"Values": ["\u672a\u547d\u540d"], "Name": "instance-name"}]}"#;
        assert_eq!(
            sha_256_hex(payload).1,
            "35e9c5b0e3ae67532d3c9f17ead6c90222632e5b1ff7f6e89887f1398934f064"
        );
    }

    #[test]
    fn test_hmac_256_hex() {
        let key = "my key";
        let payload = r#"{"Limit": 1, "Filters": [{"Values": ["\u672a\u547d\u540d"], "Name": "instance-name"}]}"#;
        assert_eq!(
            hmac_256(key.as_bytes(), payload).1,
            "34b60adbb51a1df3ebf43652cf9ef31e6dde1897b5b986bac8b245492b2bb90a"
        );
    }

    #[test]
    fn test_make_canonical_request() {
        let payload = r#"{"Limit": 1, "Filters": [{"Values": ["\u672a\u547d\u540d"], "Name": "instance-name"}]}"#;
        let request_string =
            make_canonical_request_string("lighthouse.tencentcloudapi.com", "application/json", payload);
        let expected_result = r#"POST
/

content-type:application/json
host:lighthouse.tencentcloudapi.com

content-type;host
35e9c5b0e3ae67532d3c9f17ead6c90222632e5b1ff7f6e89887f1398934f064"#;
        assert_eq!(request_string, expected_result);
    }

    #[test]
    fn test_make_string_to_sign() {
        let canonical_request = r#"POST
/

content-type:application/json; charset=utf-8
host:cvm.tencentcloudapi.com

content-type;host
35e9c5b0e3ae67532d3c9f17ead6c90222632e5b1ff7f6e89887f1398934f064"#;
        // let timestamp = Utc::now().timestamp();
        let timestamp = 1551113065;
        let string_to_sign = make_string_to_sign(canonical_request, timestamp, "2019-02-25", "lighthouse");
        let expected_result = r#"TC3-HMAC-SHA256
1551113065
2019-02-25/lighthouse/tc3_request
5ffe6a04c0664d6b969fab9a13bdab201d63ee709638e2749d62a09ca18d7031"#;
        assert_eq!(string_to_sign, expected_result);
    }

    #[test]
    fn test_make_signature_string() {
        let string_to_sign = r#"TC3-HMAC-SHA256
1551113065
2019-02-25/cvm/tc3_request
5ffe6a04c0664d6b969fab9a13bdab201d63ee709638e2749d62a09ca18d7031"#;
        let signature_string = make_signature_string(
            "Gu5t9xGARNpq86cd98joQYCN3*******",
            string_to_sign,
            "2019-02-25",
            "cvm",
        );
        let expected_result = "2230eefd229f582d8b1b891af7107b91597240707d778ab3738f756258d7652c";
        // println!("{signature_string}");
        assert_eq!(signature_string, expected_result);
    }

    #[test]
    fn test_make_authorization_string() {
        let signature_string = "2230eefd229f582d8b1b891af7107b91597240707d778ab3738f756258d7652c";
        let authorization_string = make_authorization_string(
            "AKIDz8krbsJ5yKBZQpn74WFkmLPx3*******",
            "2019-02-25",
            "cvm",
            signature_string,
        );
        assert_eq!(authorization_string, "TC3-HMAC-SHA256 Credential=AKIDz8krbsJ5yKBZQpn74WFkmLPx3*******/2019-02-25/cvm/tc3_request, SignedHeaders=content-type;host, Signature=2230eefd229f582d8b1b891af7107b91597240707d778ab3738f756258d7652c");
    }

    #[test]
    fn test_make_auth_string_all_in_one() {
        let payload = r#"{"Limit": 1, "Filters": [{"Values": ["\u672a\u547d\u540d"], "Name": "instance-name"}]}"#;
        let host = "cvm.tencentcloudapi.com";
        let content_type = "application/json; charset=utf-8";
        let timestamp = 1551113065;
        let date = "2019-02-25";
        let secret_id = "AKIDz8krbsJ5yKBZQpn74WFkmLPx3*******";
        let secret_key = "Gu5t9xGARNpq86cd98joQYCN3*******";
        let service = "cvm";
        let auth_string = make_auth_string_all_in_one(
            host,
            content_type,
            payload,
            timestamp,
            date,
            secret_id,
            secret_key,
            service,
        );
        assert_eq!(auth_string, "TC3-HMAC-SHA256 Credential=AKIDz8krbsJ5yKBZQpn74WFkmLPx3*******/2019-02-25/cvm/tc3_request, SignedHeaders=content-type;host, Signature=2230eefd229f582d8b1b891af7107b91597240707d778ab3738f756258d7652c");
    }
}
