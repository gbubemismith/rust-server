use std::net::{TcpListener, TcpStream};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            address: addr.to_owned(),
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    
                }
                Err(e) => println!("Failed to establish a connection {}", e),
            }
        }
    }
}
