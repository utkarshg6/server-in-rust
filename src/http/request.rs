use std::str;
use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Debug, Formatter};

// An HTTP Request of different Methods like GET, PUT etc.
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

// This is how type conversions are 
// meant to be implemented in Rust.
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // &[u8] is a byte slice

    // Example Request - GET /search?name=abc&sort=1 HTTP/1.1

    // for keyword is used to extend the Request data type
    // It means any function defined in this trait can now
    // be called through a variable of type Request
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i+1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
        // unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]))
        }
    }

    None
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
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}
