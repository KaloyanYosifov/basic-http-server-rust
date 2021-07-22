use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

use crate::server::Method::GET;

use super::Method;

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
    use crate::server::Method::{GET, POST};

    use crate::server::request::{Request, RequestError};

    #[test]
    fn it_can_create_a_request_from_buffer() {
        let buffer = "GET / HTTP/1.1".as_bytes();
        let request: Request = buffer.try_into().unwrap();

        assert_eq!("/", request.route);
        assert_eq!("HTTP/1.1", request.protocol);
        assert!(matches!(request.method, GET));
    }

    #[test]
    fn it_can_parse_other_request_methods() {
        let buffer = "POST / HTTP/1.1".as_bytes();
        let request: Request = buffer.try_into().unwrap();

        assert_eq!("/", request.route);
        assert_eq!("HTTP/1.1", request.protocol);
        assert!(matches!(request.method, POST));
    }

    #[test]
    fn it_returns_an_error_if_it_cannot_be_encoded() {
        let buffer: &[u8] = &[255; 3][..];
        let request: Result<Request, RequestError> = buffer.try_into();

        match request {
            Ok(_) => panic!("This should have failed!"),
            _ => assert!(true)
        }
    }
}
