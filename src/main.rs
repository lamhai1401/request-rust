use std::collections::HashMap;

extern crate requestrust;

mod client;
mod errors;
mod lib;
mod rq;

use client::Api;
use rq::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/ip".to_string();
    let mut api: Api = Request::new(url.clone());

    let resp = api.get(url.clone()).await?;
    println!("{:#?}", resp);
    Ok(())
}
