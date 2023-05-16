use actix_http::StatusCode;
use serde_derive::Deserialize;
use std::fmt;

#[derive(Debug)]
pub enum FetchError {
    /// fetch url error
    Network(reqwest::Error),
    /// http code is not valid in 200 and 300
    Http(HttpError),
    /// invalid response json
    Parse(ParseError),
}

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub message: String,
}

#[derive(Debug)]
pub struct HttpError {
    pub message: String,
    pub code: StatusCode,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub code: StatusCode,
    pub body: String,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchError::Network(err) => {
                write!(f, "Network error: {}", err.to_string())
            }
            FetchError::Http(err) => {
                write!(f, "Http error [{}]: {}", err.code, err.message)
            }
            FetchError::Parse(err) => {
                write!(
                    f,
                    "Parse error [{}]: {}. \nOrigin body: {}",
                    err.code, err.message, err.body
                )
            }
        }
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> Self {
        FetchError::Network(err)
    }
}

impl From<HttpError> for FetchError {
    fn from(err: HttpError) -> Self {
        FetchError::Http(err)
    }
}
impl From<ParseError> for FetchError {
    fn from(err: ParseError) -> Self {
        FetchError::Parse(err)
    }
}
