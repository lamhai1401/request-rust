use super::errors::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
pub type ReqResult<T> = Result<T, Error>;

#[async_trait]
pub trait Request {
    type Output;
    const URL: &'static str = "http://localhost:4000";
    const PATH: &'static str;
    // type Output;
    fn new(url: String) -> Self
    where
        Self: Sized;
    async fn get(&mut self, path: String) -> ReqResult<Self::Output>;
    // async fn get<'de, T>(&mut self, path: &'de String) -> ReqResult<T>
    // where
    //     T: Deserialize<'de>;

    async fn parsing<'de, T>(resp: &'de String) -> ReqResult<T>
    where
        T: Deserialize<'de>;
    // async fn post<T>(
    //     &mut self,
    //     url: String,
    //     header: HashMap<String, String>,
    //     body: HashMap<String, Value>,
    // ) -> ReqResult<T>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resp {
    #[serde(alias = "origin")]
    #[serde(rename = "origin")]
    origin: String,
}
