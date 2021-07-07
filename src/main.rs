use clap::{Arg, App};
use std::collections::HashMap;
use std::time::{SystemTime, Instant};
use crypto::mac::Mac;
use crypto::hmac::Hmac;
use crypto::sha1::Sha1;
use std::collections::hash_map::RandomState;

static DATA_PATTERN: &str = "%a, %d %b %Y %H:%M:%S GMT";

fn main() {
    let matches = App::new("cas-rust")
        .version("0.1")
        .args(&[
            Arg::from_usage("-k, --key <api-key> 'api key'").required_unless("file"),
            Arg::from_usage("-u, --user <api-user> 'api user'").required_unless("file"),
            Arg::from_usage("-f, --file [input] 'an optional input file to use'").required_unless_all(&["key","user"])
        ])
        .get_matches();

    println!("get matches successfully");
    if let Some(f) = matches.value_of("file") {
        println!("file: {}",f)
    }else {
        let key = matches.value_of("key").unwrap();
        let user = matches.value_of("user").unwrap();
        println!("api-user:{}, api-key:{}", user,key);
        let map = encrypt(user, key);
        for (k,v) in map {
            println!("{}:{}",k,v);
        }
    }
}

fn encrypt(user: &str, key: &str) -> HashMap<String, String, RandomState> {
    let now = chrono::prelude::Utc::now().format(DATA_PATTERN).to_string();
    let mut headers = HashMap::new();
    headers.insert(String::from("Date"),now.clone());
    let mut hmac1 = Hmac::new(Sha1::new(), key.as_bytes());
    hmac1.input(now.as_bytes());
    let encrypt = base64::encode(hmac1.result().code());
    let mut authorizationStr = String::from(user);
    authorizationStr.push_str(":");
    authorizationStr.push_str(encrypt.as_str());
    authorizationStr = base64::encode(authorizationStr.as_bytes());
    headers.insert(String::from("Authorization"),"Basic: ".to_owned() + &authorizationStr);
    return headers;
}