struct Server {
    address: String,
}

impl Server {
    fn new(addr: &str) -> Self {
        Self {
            address: addr.to_owned(),
        }
    }

    fn run(&self) {}
}

fn main() {}
