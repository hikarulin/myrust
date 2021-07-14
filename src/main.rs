use clap::{App, Arg};

mod common;

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
        let map = common::encrypt(user, key);
        println!("api-user:{}, api-key:{}", user,key);
        for (k,v) in map {
            println!("{}:{}",k,v);
        }
    }
}