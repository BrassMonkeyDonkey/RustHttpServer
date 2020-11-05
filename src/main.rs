fn main() {
    println!("Starting server...");
    let server = Server::new("127.0.0.1:8088".to_string());
    server.run();    
}

struct Server {
    addr: String
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    
    fn run(self) {
        // Do something and be disposed at the end of it!
        println!("Server started on: {}", self.addr);
    }
}
