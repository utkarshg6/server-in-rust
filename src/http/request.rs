use super::method::Method;
use std::convert::TryFrom;

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
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
