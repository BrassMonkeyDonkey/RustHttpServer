#![allow(dead_code)]
#![allow(non_snake_case)]

use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    println!("Starting server...");
    let server = Server::new("127.0.0.1:8088".to_string());
    server.run(WebsiteHandler); 
}
