use crate::http::HttpMethod;

use super::http::{HttpRequest, HttpResponse, StatusCode};
use super::server::Handler;
use std::{fs, path};

pub struct WebsideHandler {
    public_path: String,
}
impl WebsideHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    print!("Directory Traversal Attack attempted! {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsideHandler {
    fn handle_request(&mut self, request: &HttpRequest) -> HttpResponse {
        match request.method() {
            HttpMethod::GET => match request.path() {
                "/" => HttpResponse::new(StatusCode::Ok, (self.read_file("index.html"))),
                "/hello" => HttpResponse::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => HttpResponse::new(StatusCode::Ok, Some(content)),
                    None => HttpResponse::new(StatusCode::NotFound, None),
                },
            },
            _ => HttpResponse::new(StatusCode::NotFound, None),
        }
    }
}
