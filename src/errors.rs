use reqwest::Error as ReqErr;
use serde_json::Error as JsonErr;
use snafu::*;
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[snafu(display("Send request err {}", details))]
    #[non_exhaustive]
    RequestErr { details: String },

    #[snafu(display("Json err {}", details))]
    #[non_exhaustive]
    JsonErr { details: String },
}

impl From<ReqErr> for Error {
    fn from(err: ReqErr) -> Error {
        Error::RequestErr {
            details: err.to_string(),
        }
    }
}

impl From<JsonErr> for Error {
    fn from(err: JsonErr) -> Error {
        Error::JsonErr {
            details: err.to_string(),
        }
    }
}
