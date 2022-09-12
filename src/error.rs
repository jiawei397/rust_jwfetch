use std::fmt;

pub enum ErrType {
    /// fetch url error
    NetworkErr,
    /// http code is not valid in 200 and 300
    HttpErr,
    /// invalid response json
    ParseJSONErr,
}

pub struct FetchError {
    pub message: String,
    pub code: Option<u16>,
    pub err_type: ErrType,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
