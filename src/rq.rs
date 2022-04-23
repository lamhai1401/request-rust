use super::errors::Error;
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;

pub type ReqResult<T> = Result<T, Error>;

#[async_trait]
pub trait Request {
    type Output;
    // const URL: &'static str;
    // const PATH: &'static str;
    fn new(url: String) -> Self
    where
        Self: Sized;
    async fn get(&mut self, path: String) -> ReqResult<Self::Output>;

    async fn parsing<'de, T>(resp: &'de String) -> ReqResult<T>
    where
        T: Deserialize<'de>;
    async fn post<T>(
        &mut self,
        url: String,
        header: &'static HashMap<String, String>,
        body: String,
    ) -> ReqResult<Self::Output>;
}
