use jwfetch::{request, FetchError, Method, RequestConfig};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
}

pub async fn get_user_info() -> Result<UserInfo, FetchError> {
    request::<UserInfo>(RequestConfig {
        url: "user/info".to_owned(),
        method: Method::GET,
        base_url: Some("http://127.0.0.1:4523/m1/595662-0-default/api/".to_string()),
        origin_headers: None,
        headers: None,
        data: None,
        timeout: None,
        extra_header_keys: Some(vec!["user-agent", "cookie", "referer"]),
    })
    .await
}

// Short example of a POST request with form data.
//
// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[tokio::main]
async fn main() {
    let info = get_user_info().await;
    // println!("{:?}", info);
    match info {
        Ok(info) => {
            println!("{:?}", info);
        }
        Err(e) => {
            match e {
                FetchError::Http(err) => {
                    println!("Http error [{}]: {}", err.code, err.message);
                }
                FetchError::Parse(err) => {
                    println!(
                        "Parse error [{}]: {}. \nOrigin body: {}",
                        err.code, err.message, err.body
                    );
                }
                FetchError::Network(network_error) => {
                    println!("network_error: {}", network_error);
                }
            }
            // Or you can  simple print error
            // println!("{}", e);
        }
    }
}
