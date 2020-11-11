#![allow(dead_code)]
#![allow(non_snake_case)]

use server::Server;

mod server;
mod http;

fn main() {
    println!("Starting server...");
    let server = Server::new("127.0.0.1:8088".to_string());
    server.run(); 
}
