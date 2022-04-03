use snafu::*;

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[snafu(display("Send get request err {}", details))]
    #[non_exhaustive]
    GetRequestErr { details: String, status_code: i32 },

    #[snafu(display("Send post request err {}", details))]
    #[non_exhaustive]
    PostRequestErr { details: String, status_code: i32 },
}

// impl From<TungErr> for Error {
//     fn from(err: TungErr) -> Error {
//         Error::WssConnectionErr {
//             details: err.to_string(),
//         }
//     }
// }
