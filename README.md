# jwfetch

Package reqwest with trace.

## Example

```rust
use jwfetch::{request, RequestConfig, ServiceError, ActixHeaderMap, Method};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub avatar: String,
}

pub async fn get_user_info(origin_headers: ActixHeaderMap) -> Result<UserInfo, ServiceError> {
    request::<UserInfo>(&RequestConfig {
        url: "user/userinfo",
        method: Method::GET,
        base_url: Some("https://api.github.com/api/"),
        origin_headers: Some(origin_headers),
        headers: None,
        data: None,
        timeout: None,
        extra_header_keys: Some(vec!["user-agent", "cookie", "referer"]),
    })
    .await
}
```
