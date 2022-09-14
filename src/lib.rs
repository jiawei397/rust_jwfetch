pub mod ajax;
pub mod error;

pub use self::ajax::{get, post, request, RequestConfig};
pub use self::error::{ErrType, FetchError};
pub use actix_http::header::{HeaderMap as ActixHeaderMap, HeaderName};
pub use http::Method;
pub use reqwest::header::HeaderMap;
pub use std::time::Duration;
