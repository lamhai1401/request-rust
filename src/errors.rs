use snafu::*;

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[snafu(display("Connecting to Wss Err {}", details))]
    #[non_exhaustive]
    ReuqestErr { details: String, status_code: i32 },
}

// impl From<TungErr> for Error {
//     fn from(err: TungErr) -> Error {
//         Error::WssConnectionErr {
//             details: err.to_string(),
//         }
//     }
// }
