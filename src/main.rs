#![allow(dead_code)]
use server::Server;
use std::{default, env};
use website_handler::WebsideHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("localhost:8080".to_string());
    server.run(WebsideHandler::new(public_path));
}
