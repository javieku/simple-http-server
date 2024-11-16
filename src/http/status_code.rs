use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn as_str(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}
