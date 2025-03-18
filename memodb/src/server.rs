use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(host: &str, port: usize) -> Self {
        let address = format!("{}:{}", host, port);
        Server { address }
    }

    pub fn listen(&self) -> Result<(), &'static str> {
        let listener = TcpListener::bind(&self.address);

        Ok(())
    }
}
