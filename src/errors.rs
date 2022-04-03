use reqwest::Error as ReqErr;
use snafu::*;
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[snafu(display("Send request err {}", details))]
    #[non_exhaustive]
    RequestErr { details: String },
}

impl From<ReqErr> for Error {
    fn from(err: ReqErr) -> Error {
        Error::RequestErr {
            details: err.to_string(),
        }
    }
}
