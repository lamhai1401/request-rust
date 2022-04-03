use super::{
    errors::Error,
    rq::{ReqResult, Request},
};
use async_trait::async_trait;
use reqwest::{header, Client, StatusCode};
use serde_json::json;

use std::collections::HashMap;

pub struct Api {
    url: String,
    client: Client,
}

#[async_trait]
impl Request for Api {
    const PATH: &'static str = "/api";
    fn new(url: String) -> Self {
        Api {
            url,
            client: Client::new(),
        }
    }

    async fn get(&mut self, uri: String) -> ReqResult {
        // let url = format!(
        //     "{}{}{}",
        //     self.url.clone(),
        //     Self::PATH.clone(),
        //     uri.to_owned()
        // );
        let resp = self.client.get(uri.to_owned()).send().await?.text().await?;
        match serde_json::from_str::<HashMap<String, String>>(resp.as_str()) {
            Ok(data) => Ok(data),
            Err(er) => Err(Error::JsonErr {
                details: er.to_string(),
            }),
        }
    }
}
