use super::{client::Api, rq::Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resp {
    #[serde(alias = "origin")]
    #[serde(rename = "origin")]
    pub origin: String,
}

// impl Clone for Resp {
//     fn clone(&self) -> Self {
//         Resp {
//             origin: self.origin.clone(),
//         }
//     }
// }

#[tokio::test]
async fn test_get_api() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/ip".to_string();
    let mut api: Api = Request::new(url.clone());

    let resp = api.get("".to_string()).await?;

    let result = Api::parsing::<Resp>(&resp).await?;

    // assert_eq!(result.origin, "113.161.39.115");
    assert_ne!(result.origin, "");

    Ok(())
}
