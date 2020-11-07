pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    
    pub fn run(self) {
        // Do something and be disposed at the end of it!
        println!("Server started on: {}", self.addr);
    }
}