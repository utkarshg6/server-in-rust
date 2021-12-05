use std::fmt::{Display, Formatter, Result as FmtResult};

// The below derive statment will copy the contents of enum
// and make a trivial clone out of it. It'll help us to end 
// the restrictions of referencing errors with the enum.
// Try commenting the line and see the error for *self as u16
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
