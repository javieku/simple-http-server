use std::str::FromStr;

#[derive(Debug)]
pub enum HttpMethod {
    POST,
    PUT,
    GET,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for HttpMethod {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "GET" => Ok(HttpMethod::GET),
            "HEAD" => Ok(HttpMethod::HEAD),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "TRACE" => Ok(HttpMethod::TRACE),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
