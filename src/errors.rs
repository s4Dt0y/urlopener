use std::fmt;

#[derive(Debug, Clone)]
pub enum UrlError {
    OpenUrlFailed,
}

impl fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           UrlError::OpenUrlFailed => write!(f, "Error 1"),
       }
    }
}