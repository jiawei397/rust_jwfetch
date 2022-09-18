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

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct FetchError {
    pub message: String,
    pub code: Option<u16>,
    pub err_type: ErrType,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err_type {
            ErrType::NetworkErr => {
                write!(f, "network error, {}", self.message)
            }
            ErrType::HttpErr => {
                write!(f, "http error [{}], {}", self.code.unwrap(), self.message)
            }
            ErrType::ParseJSONErr => {
                write!(
                    f,
                    "parse json error [{}], {}",
                    self.code.unwrap(),
                    self.message
                )
            }
        }
    }
}
