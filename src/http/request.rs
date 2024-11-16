use super::method::HttpMethod;
use super::method::MethodError;
use super::QueryString;

use core::str;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct HttpRequest<'buf> {
    path: &'buf str,
    query: Option<QueryString<'buf>>,
    method: HttpMethod,
}

impl<'buf> HttpRequest<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn query(&self) -> Option<&QueryString> {
        self.query.as_ref()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (idx, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' || char == '\n' || char == '\t' {
            return Some((&request[..idx], &request[idx + 1..]));
        }
    }
    None
}

impl<'buf> TryFrom<&'buf [u8]> for HttpRequest<'buf> {
    type Error = ParseError;

    fn try_from(data: &'buf [u8]) -> Result<HttpRequest<'buf>, Self::Error> {
        let request_buffer = str::from_utf8(data)?;

        let (method, request) = get_next_word(request_buffer).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: HttpMethod = method.parse()?;

        let mut query = None;
        if let Some(idx) = path.find('?') {
            query = Some(QueryString::from(&path[idx + 1..]));
            path = &path[..idx];
        }

        Ok(HttpRequest {
            path: path,
            query: query,
            method: method,
        })
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidPath,
    InvalidQuery,
    InvalidProtocol,
    InvalidEncoding,
    InvalidHeader,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Parse error: {}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_err: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}
impl From<MethodError> for ParseError {
    fn from(_err: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid request",
            ParseError::InvalidMethod => "Invalid method",
            ParseError::InvalidPath => "Invalid path",
            ParseError::InvalidQuery => "Invalid query",
            ParseError::InvalidProtocol => "Invalid protocol",
            ParseError::InvalidEncoding => "Invalid encoding",
            ParseError::InvalidHeader => "Invalid header",
        }
    }
}

impl Error for ParseError {}
