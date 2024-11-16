use crate::http::{request::ParseError, HttpRequest, HttpResponse, StatusCode};
use std::{io::Read, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::new(StatusCode::Ok, Some("<h1>It's WORKS!</h1>".to_string()))
    }

    fn handle_bad_request(&mut self, e: &ParseError) -> HttpResponse {
        println!("Failed to parse request: {}", e);
        HttpResponse::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }
    pub fn run(self, mut handler: impl Handler) {
        println!("Server listening on {}", self.address);
        let listener = TcpListener::bind(self.address).unwrap();
        let mut buffer = [0; 1024];
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => match stream.read(&mut buffer) {
                    Ok(_) => {
                        let request = HttpRequest::try_from(&buffer[..]);
                        println!("Received request: {:?}", request);
                        let response = match request {
                            Ok(r) => handler.handle_request(&r),
                            Err(e) => handler.handle_bad_request(&e),
                        };
                        if let Err(e) = response.send(&mut stream) {
                            println!("Failed to send response: {}", e);
                        }
                    }
                    Err(e) => println!("Failed to read from connection:{}", e),
                },
                Err(e) => println!("Failed to estabilish a connection: {}", e),
            }
        }
    }
}
