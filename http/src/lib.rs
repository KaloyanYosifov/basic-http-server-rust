use std::net::TcpListener;
use std::io::{Read, Write};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::convert::TryInto;
use crate::request::{Request, RequestError};
use crate::response::{Response, StatusCode};
use crate::request_handler::RequestHandler;
use std::panic::resume_unwind;

pub mod route;
pub mod query_params;
pub mod request;
pub mod response;
pub mod request_handler;

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
            _ => Err(MethodParseError::InvalidMethod(value.to_string()))
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

    pub fn listen(&self, handler: impl RequestHandler) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;

        for stream in listener.incoming() {
            let mut input = [0; 4096];
            let mut stream = stream?;

            stream.read(&mut input)?;

            let request_resolver: Result<Request, RequestError> = input[..].try_into();

            match request_resolver {
                Ok(request) => handler.handle(&request).send(&mut stream),
                Err(error) => handler.handle_error(&error).send(&mut stream),
            }
        }

        Ok(())
    }
}
