use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    address: String,
}

impl Server {
    pub fn bind(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;

        for stream in listener.incoming() {
            let mut input = [0; 20];

            stream?.read(&mut input)?;

            print!("{}", std::str::from_utf8(&input).unwrap())
        }

        Ok(())
    }
}
