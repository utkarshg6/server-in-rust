use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Debug, Formatter};

// An HTTP Request of different Methods like GET, PUT etc.
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// This is how type conversions are 
// meant to be implemented in Rust.
impl TryFrom<&[u8]> for Request {
    // &[u8] is a byte slice

    // for keyword is used to extend the Request data type
    // It means any function defined in this trait can now
    // be called through a variable of type Request
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match(self) {
            Self::InvalidRequest => "InvalidRequest"
            Self::InvalidEncoding => "InvalidEncoding"
            Self::InvalidProtocol => "InvalidProtocol"
            Self::InvalidMethod => "InvalidMethod"
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}
