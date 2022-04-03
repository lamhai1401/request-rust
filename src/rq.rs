use super::errors::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub type ReqResult = Result<Resp, Error>;

#[async_trait]
pub trait Request {
    const URL: &'static str = "http://localhost:4000";
    const PATH: &'static str;
    fn new(url: String) -> Self
    where
        Self: Sized;
    async fn get(&mut self, path: String) -> ReqResult;
    // fn post(&mut self, body: String) -> ReqResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resp {
    #[serde(alias = "origin")]
    #[serde(rename = "origin")]
    origin: String,
}
