use std::convert::TryFrom;
use std::io::{Write, Read};
use std::net::TcpListener;

use crate::http::{Request, Response, StatusCode};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Server started on: {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {                                    
                                    dbg!(request);

                                    let response = Response::new(StatusCode::Ok, Some("<h1>I WON!</h1>".to_string()));
                                    write!(stream, "{}", response);
                                },
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
