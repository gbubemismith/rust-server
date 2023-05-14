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
    }
}
