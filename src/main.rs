use std::collections::HashMap;

use clap::{App, Arg};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;

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

    if let Some(f) = matches.value_of("file") {
        println!("file: {}",f)
    }else {
        let key = matches.value_of("key").unwrap();
        let user = matches.value_of("user").unwrap();
        let map = encrypt(user, key);
        println!("api-user:{}, api-key:{}", user,key);
        for (k,v) in map {
            println!("{}:{}",k,v);
        }
    }
}

fn encrypt(user: &str, key: &str) -> HashMap<String, String> {
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

fn hmac_sha1(key: &[u8],value: &[u8]) -> String {
    let mut hmac1 = Hmac::new(Sha1::new(), key);
    hmac1.input(value);
    base64::encode(hmac1.result().code())
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // let mut number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("{:?} The largest number is {}", number_list, result);
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
    // return;
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}