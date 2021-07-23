use std::net::TcpListener;
use std::io::Read;
use std::fmt::{Display, Formatter};
use std::convert::TryFrom;
use crate::server::MethodParseError::InvalidMethod;

mod request;

#[derive(Debug)]
pub enum MethodParseError {
    InvalidMethod
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    OPTIONS,
    PUT,
    PATCH,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<&str> for Method {
    type Error = MethodParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(InvalidMethod)
        }
    }
}

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
