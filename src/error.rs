use actix_http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum ErrType {
    /// fetch url error
    NetworkErr,
    /// http code is not valid in 200 and 300
    HttpErr,
    /// invalid response json
    ParseJSONErr,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CustomError {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct FetchError {
    pub message: String,
    pub code: Option<StatusCode>,
    pub err_type: ErrType,
    pub body: Option<String>,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err_type {
            ErrType::NetworkErr => {
                write!(f, "Network error, {}", self.message)
            }
            ErrType::HttpErr => {
                write!(f, "Http error [{}], {}", self.code.unwrap(), self.message)
            }
            ErrType::ParseJSONErr => {
                write!(
                    f,
                    "Parse json error [{}], {}. \nThe origin body is: {}",
                    self.code.unwrap(),
                    self.message,
                    self.body.as_ref().unwrap()
                )
            }
        }
    }
}
