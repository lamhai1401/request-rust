use super::{
    errors::Error,
    rq::{ReqResult, Request},
};
use async_trait::async_trait;
use reqwest::{header, Client, StatusCode};
use serde::{Deserialize, Serialize};

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

    // async fn parsing<T>(resp: String) -> ReqResult<T> {
    //     match serde_json::from_str::<T>(resp.as_str()) {
    //         Ok(data) => Ok(data),
    //         Err(er) => Err(Error::JsonErr {
    //             details: er.to_string(),
    //         }),
    //     }
    // }

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

    // async fn post(
    //     &mut self,
    //     uri: String,
    //     header: HashMap<String, String>,
    //     body: HashMap<String, Value>,
    // ) -> ReqResult {
    //     let url = format!(
    //         "{}{}{}",
    //         self.url.clone(),
    //         Self::PATH.clone(),
    //         uri.to_owned()
    //     );

    //     // parsing body
    //     let re_body = json!(body);

    //     // parsing header
    //     let headers = header::HeaderMap::new();
    //     let resp = self
    //         .client
    //         .post(url.to_owned())
    //         .body(re_body.to_string())
    //         .headers(headers)
    //         .send()
    //         .await?
    //         .text()
    //         .await?;

    //     match serde_json::from_str::<Resp>(resp.as_str()) {
    //         Ok(data) => Ok(data),
    //         Err(er) => Err(Error::JsonErr {
    //             details: er.to_string(),
    //         }),
    //     }
    // }
}
