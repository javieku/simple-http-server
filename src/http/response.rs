use super::status_code::StatusCode;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status: StatusCode, body: Option<String>) -> Self {
        Response { status, body }
    }
    pub fn send(&self, stream: &mut impl Write) -> io::Result<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status,
            self.status.as_str(),
            body
        )
    }
}
