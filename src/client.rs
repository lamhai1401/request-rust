use super::{
    errors::Error,
    rq::{ReqResult, Request},
};
use async_trait::async_trait;
use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use std::collections::HashMap;

pub struct Api {
    url: String,
    client: Client,
}

#[async_trait]
impl Request for Api {
    const PATH: &'static str = "/api";
    type Output = String;
    fn new(url: String) -> Self {
        Api {
            url,
            client: Client::new(),
        }
    }

    async fn parsing<'de, T>(resp: &'de String) -> ReqResult<T>
    where
        T: Deserialize<'de>,
    {
        match serde_json::from_str::<T>(resp.as_str()) {
            Ok(data) => Ok(data),
            Err(er) => Err(Error::JsonErr {
                details: er.to_string(),
            }),
        }
    }

    async fn get(&mut self, path: String) -> ReqResult<Self::Output> {
        let url = format!(
            "{}{}{}",
            self.url.clone(),
            Self::PATH.clone(),
            path.to_owned()
        );

        let resp = self.client.get(url.to_owned()).send().await?.text().await?;
        Ok(resp)
    }

    async fn post<T>(
        &mut self,
        url: String,
        header: &'static HashMap<String, String>,
        body: String,
    ) -> ReqResult<Self::Output> {
        let url = format!(
            "{}{}{}",
            self.url.clone(),
            Self::PATH.clone(),
            url.to_owned()
        );

        let mut headers = HeaderMap::new();
        headers.insert("Content-type", "application/json".parse().unwrap());

        for (key, val) in header.iter() {
            headers.insert(key.as_str(), val.parse().unwrap());
        }

        let resp = self
            .client
            .post(url.to_owned())
            .body(body)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }
}
