use myrust::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::time::Duration;
use std::str::FromStr;
use std::net::SocketAddr;

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
        .build()
        .unwrap();
    let resp = httpClient.get("https://ms.quantil.com").send().await?.text().await?;
    println!("resp:{}",resp);
    Ok(())
}