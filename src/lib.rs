pub mod ajax;
pub mod error;

pub use self::{
    ajax::{get, post, request, BaseRequestConfig, RequestConfig},
    error::{CustomError, FetchError, HttpError, ParseError},
};
pub use actix_http::{
    header::{HeaderMap as ActixHeaderMap, HeaderName},
    StatusCode,
};
pub use http::Method;
pub use reqwest::header::{HeaderMap, HeaderValue};
pub use std::time::Duration;
