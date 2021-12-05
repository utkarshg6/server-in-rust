// This file exposes submodules or entities inside of them.

pub use method::Method; // Exposing only enum Method
pub use query_string::{QueryString, Value as QueryStringValue};
pub use request::ParseError; // Exposing only enum ParseError
pub use request::Request; // Exposing only struct Request
pub use response::Response; // Exposing only struct Response
pub use status_code::StatusCode;

pub mod method; // Exposing the whole submodule method
pub mod query_string; // Now only rust compiler will compile this file
pub mod request; // Exposing the whole submodule request
pub mod response;
pub mod status_code;