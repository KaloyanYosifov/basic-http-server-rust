use std::convert::TryFrom;
use std::str::Utf8Error;
use std::error::Error;
use std::fmt::{Display, Formatter};
use super::Method;
use crate::server::Method::GET;

#[derive(Debug)]
pub enum RequestError {
    FailedToParse,
    InvalidEncoding,
    InvalidMethod(String),
    InvalidProtocol(String),
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred {}", self.to_string())
    }
}

impl Error for RequestError {}

impl From<Utf8Error> for RequestError {
    fn from(_: Utf8Error) -> Self {
        RequestError::InvalidEncoding
    }
}

pub struct Request {
    route: String,
    protocol: String,
    method: Method,
}

impl Request {
    pub fn new(
        method: Method,
        protocol: String,
        route: String,
    ) -> Self {
        Request {
            route,
            method,
            protocol,
        }
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = RequestError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let contents = std::str::from_utf8(&buffer)?;
        let request = Request::new(
            Method::GET,
            "HTTP/1.1".to_string(),
            "/".to_string(),
        );

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use crate::server::request::Request;

    #[test]
    fn it_can_create_a_request_from_buffer() {
        let buffer = "GET / HTTP/1.1".as_bytes();
        let request: Request = buffer.try_into().unwrap();

        assert_eq!("/", request.route);
        assert_eq!("HTTP/1.1", request.protocol);
        matches!(request.method, super::super::Method::GET);
    }
}
