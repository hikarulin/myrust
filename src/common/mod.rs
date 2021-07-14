use crypto::hmac::Hmac;
use crypto::sha1::Sha1;
use std::collections::HashMap;
use crypto::mac::Mac;

static DATA_PATTERN: &str = "%a, %d %b %Y %H:%M:%S GMT";

pub fn encrypt(user: &str, key: &str) -> HashMap<String, String> {
    let now = chrono::prelude::Utc::now().format(DATA_PATTERN).to_string();
    let encrypt = hmac_sha1(key.as_bytes(),now.as_bytes());
    let mut authorization_str = String::from(user) + ":" + encrypt.as_str();
    authorization_str = base64::encode(authorization_str.as_bytes());

    let tuples = vec![(String::from("Date"),now),(String::from("Authorization"),"Basic ".to_owned() + &authorization_str)];
    let headers = tuples.into_iter().collect();
    // let mut headers = HashMap::new();
    // headers.insert(String::from("Date"),now);
    // headers.insert(String::from("Authorization"),"Basic: ".to_owned() + &authorization_str);
    return headers;
}

pub fn hmac_sha1(key: &[u8],value: &[u8]) -> String {
    let mut hmac1 = Hmac::new(Sha1::new(), key);
    hmac1.input(value);
    base64::encode(hmac1.result().code())
}