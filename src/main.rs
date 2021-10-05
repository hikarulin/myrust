use myrust::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::time::Duration;
use std::str::FromStr;
use std::net::{SocketAddr, SocketAddrV4};
use reqwest::Proxy;

mod common;
mod args;
mod concurrent;
mod duotai;
mod http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let httpClient = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(60))
        .connection_verbose(true)
        .proxy(Proxy::http("http://127.0.0.1:1080").unwrap())
        .build()
        .unwrap();
    let resp = httpClient.get("https://ms.quantil.com/api/v2").send().await?.text().await?;
    println!("resp:{}",resp);
    let sum = common::math::add(1,2);
    // let sum = common::math::internal_add(1,2); //compile error
    Ok(())
}