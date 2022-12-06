# jwfetch

Package reqwest with trace.

## Example

```rust
use jwfetch::{request, get, post, BaseRequestConfig, RequestConfig, FetchError, ActixHeaderMap, Method};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub avatar: String,
}

pub async fn get_user_info(origin_headers: ActixHeaderMap) -> Result<UserInfo, FetchError> {
    request::<UserInfo>(RequestConfig {
        url: "user/userinfo".to_string(),
        method: Method::GET,
        base_url: Some("https://api.github.com/api/".to_string()),
        origin_headers: Some(origin_headers),
        headers: None,
        data: None,
        timeout: None,
        extra_header_keys: Some(vec!["user-agent", "cookie", "referer"]),
    })
    .await
}
```

Or you can use `get`, `post` with `BaseRequestConfig` instead.
