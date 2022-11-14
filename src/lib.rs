pub mod ajax;
pub mod error;

pub use self::ajax::{get, post, request, BaseRequestConfig, RequestConfig};
pub use self::error::{CustomError, FetchError, HttpError, ParseError};
pub use actix_http::header::{HeaderMap as ActixHeaderMap, HeaderName};
pub use actix_http::StatusCode;
pub use http::Method;
pub use reqwest::header::HeaderMap;
pub use std::time::Duration;
