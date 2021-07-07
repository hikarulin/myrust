extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("cas-rust")
        .version("0.1")
        .args(&[
            Arg::from_usage("-k, --key <api-key> 'api key'").required_unless("file"),
            Arg::from_usage("-u, --user <api-user> 'api user'").required_unless("file"),
            Arg::from_usage("-f, --file [input] 'an optional input file to use'").required_unless_all(&["key","user"])
        ])
        .get_matches();

    let key = matches.value_of("key");
    match key {
        None => println!("not found arg: key"),
        Some(s) => println!("args key={}",s)
    }
}