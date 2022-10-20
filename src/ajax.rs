use crate::error::{CustomError, ErrType, FetchError};
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
    pub url: String,
    pub method: Method,
    pub base_url: Option<&'static str>,
    pub headers: Option<HeaderMap>,
    pub data: Option<String>,
    pub timeout: Option<Duration>,
    pub origin_headers: Option<ActixHeaderMap>,
    /// 需要配合origin_headers使用，指从原始headers中读取一些keys传递
    pub extra_header_keys: Option<Vec<&'static str>>,
}

#[derive(Debug)]
pub struct BaseRequestConfig {
    pub base_url: Option<&'static str>,
    pub headers: Option<HeaderMap>,
    pub data: Option<String>,
    pub timeout: Option<Duration>,
    pub origin_headers: Option<ActixHeaderMap>,
    pub extra_header_keys: Option<Vec<&'static str>>,
}

fn get_error_message(err: &str) -> String {
    match serde_json::from_str::<CustomError>(err) {
        Ok(err) => err.message,
        Err(err) => err.to_string(),
    }
}

/// fetch from remote url
pub async fn request<T>(options: &RequestConfig) -> Result<T, FetchError>
where
    T: serde::Serialize,
    for<'de2> T: serde::Deserialize<'de2>,
{
    let client = reqwest::Client::new();
    let mut url = match &options.base_url {
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
    if &options.method == Method::GET || &options.method == Method::DELETE {
        if let Some(extra_str) = &options.data {
            if url.ends_with("?") {
                url.push_str(format!("&{}", &extra_str).as_str());
            } else {
                url.push_str(format!("?{}", &extra_str).as_str());
            }
        }
    }

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
    if &options.method == Method::POST || &options.method == Method::PUT {
        if !new_headers.contains_key("content-type") {
            new_headers.insert(
                HeaderName::from_static("content-type"),
                "application/json; charset=utf-8".parse().unwrap(),
            );
        }
        if let Some(body) = &options.data {
            // let body = serde_json::to_string(data).unwrap(); //TODO 怎么动态定义这个类型没有头绪
            builder = builder.body(body.to_owned());
        }
    }
    builder = builder.headers(new_headers);

    match builder.send().await {
        Ok(resp) => {
            let http_code = resp.status();
            let ret_body = &resp.text().await.unwrap_or_default();
            if http_code.is_success() {
                let body = ret_body.as_str();
                match serde_json::from_str::<T>(body) {
                    Ok(body) => Ok(body),
                    Err(err) => Err(FetchError {
                        message: get_error_message(&err.to_string()),
                        code: Some(http_code),
                        err_type: ErrType::ParseJSONErr,
                        body: Some(body.to_string()),
                    }),
                }
            } else {
                Err(FetchError {
                    message: get_error_message(ret_body),
                    err_type: ErrType::HttpErr,
                    code: Some(http_code),
                    body: None,
                })
            }
        }
        Err(err) => Err(FetchError {
            message: get_error_message(&err.to_string()),
            err_type: ErrType::NetworkErr,
            code: None,
            body: None,
        }),
    }
}

pub async fn get<T>(url: String, options: &BaseRequestConfig) -> Result<T, FetchError>
where
    T: serde::Serialize,
    for<'de2> T: serde::Deserialize<'de2>,
{
    request(&RequestConfig {
        method: Method::GET,
        url,
        base_url: options.base_url,
        headers: options.headers.to_owned(),
        data: options.data.to_owned(),
        timeout: options.timeout,
        origin_headers: options.origin_headers.to_owned(),
        extra_header_keys: options.extra_header_keys.to_owned(),
    })
    .await
}

pub async fn post<T>(url: String, options: &BaseRequestConfig) -> Result<T, FetchError>
where
    T: serde::Serialize,
    for<'de2> T: serde::Deserialize<'de2>,
{
    request(&RequestConfig {
        method: Method::POST,
        url,
        base_url: options.base_url,
        headers: options.headers.to_owned(),
        data: options.data.to_owned(),
        timeout: options.timeout,
        origin_headers: options.origin_headers.to_owned(),
        extra_header_keys: options.extra_header_keys.to_owned(),
    })
    .await
}
