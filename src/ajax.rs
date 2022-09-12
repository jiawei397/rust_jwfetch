use crate::error::{ErrType, FetchError};
use actix_http::header::{HeaderMap as ActixHeaderMap, HeaderName};
use http::Method;
use reqwest::{self, header::HeaderMap};
use std::time::Duration;

pub fn add_trace_header(
    origin_headers: &ActixHeaderMap,
    extra_header_keys: &Option<Vec<&'static str>>,
) -> HeaderMap {
    let trace_keys = [
        "x-request-id",
        "x-b3-traceid",
        "x-b3-spanid",
        "x-b3-parentspanid",
        "x-b3-sampled",
        // "user-agent",
        // "cookie",
        // "referer",
    ];
    let header_keys = match extra_header_keys {
        Some(keys) => [keys.to_owned(), trace_keys.to_vec()].concat(),
        None => trace_keys.to_vec(),
    };
    HeaderMap::from_iter(header_keys.into_iter().filter_map(|key| {
        let hn = HeaderName::from_static(key);
        match origin_headers.get(&hn) {
            Some(v) => Some((hn, v.clone())),
            None => None,
        }
    }))
}

#[derive(Debug)]
pub struct RequestConfig {
    pub url: &'static str,
    pub method: Method,
    pub base_url: Option<&'static str>,
    pub headers: Option<HeaderMap>,
    pub data: Option<String>,
    pub timeout: Option<Duration>,
    pub origin_headers: Option<ActixHeaderMap>,
    /// 需要配合origin_headers使用，指从原始headers中读取一些keys传递
    pub extra_header_keys: Option<Vec<&'static str>>,
}

/// fetch from remote url
pub async fn request<T>(options: &RequestConfig) -> Result<T, FetchError>
where
    T: serde::Serialize,
    for<'de2> T: serde::Deserialize<'de2>,
{
    let client = reqwest::Client::new();
    let url = match &options.base_url {
        Some(base_url) => {
            let base_url = if base_url.ends_with("/") {
                &base_url[0..&base_url.len() - 1]
            } else {
                &base_url
            };

            let last_url = if options.url.starts_with("/") {
                &options.url[1..]
            } else {
                &options.url
            };
            format!("{}/{}", base_url, last_url).to_string()
        }
        None => options.url.to_string(),
    };

    let mut builder = client.request(options.method.to_owned(), &url).timeout(
        options
            .timeout
            .unwrap_or_else(|| Duration::from_secs(60 * 2)),
    );

    let mut new_headers = match &options.origin_headers {
        Some(origin_headers) => add_trace_header(origin_headers, &options.extra_header_keys),
        None => HeaderMap::new(),
    };

    if let Some(headers) = &options.headers {
        new_headers.extend(headers.clone());
    }
    builder = builder.headers(new_headers);

    if let Some(body) = &options.data {
        // let body = serde_json::to_string(data).unwrap(); //TODO 怎么动态定义这个类型没有头绪
        builder = builder.body(body.to_owned());
    }

    match builder.send().await {
        Ok(resp) => {
            let http_code = resp.status().as_u16();
            let ret_body = &resp.text().await.unwrap_or_default();
            if http_code >= 200 && http_code < 300 {
                match serde_json::from_str::<T>(ret_body.as_str()) {
                    Ok(body) => Ok(body),
                    Err(err) => Err(FetchError {
                        message: err.to_string(),
                        code: Some(http_code),
                        err_type: ErrType::ParseJSONErr,
                    }),
                }
            } else {
                Err(FetchError {
                    message: ret_body.to_string(),
                    err_type: ErrType::HttpErr,
                    code: Some(http_code),
                })
            }
        }
        Err(err) => Err(FetchError {
            message: err.to_string(),
            err_type: ErrType::NetworkErr,
            code: None,
        }),
    }
}
