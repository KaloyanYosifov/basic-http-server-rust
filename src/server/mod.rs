use std::net::TcpListener;
use std::io::Read;
use std::fmt::{Display, Formatter};
use crate::server::MethodParseError::InvalidMethod;
use std::str::FromStr;
use crate::server::request::{Request, RequestError};
use std::convert::TryInto;

mod route;
mod query_params;
mod request;

#[derive(Debug)]
pub enum MethodParseError {
    InvalidMethod(String)
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

impl FromStr for Method {
    type Err = MethodParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(InvalidMethod(value.to_string()))
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
            let mut input = [0; 4096];

            stream?.read(&mut input)?;

            let request_resolver: Result<Request, RequestError> = input[..].try_into();

            match request_resolver {
                Ok(request) => {
                    println!("{:?}", request)
                }
                _ => panic!("Something went wrong! {:?}", std::str::from_utf8(&input).unwrap())
            }
        }

        Ok(())
    }
}
