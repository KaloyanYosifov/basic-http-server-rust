use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

use super::Method;
use crate::server::request::RequestError::{FailedToParse, InvalidMethod};
use crate::server::MethodParseError;

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

impl From<MethodParseError> for RequestError {
    fn from(error: MethodParseError) -> Self {
        match error {
            MethodParseError::InvalidMethod(value) => InvalidMethod(value)
        }
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
        let contents: Vec<&str> = std::str::from_utf8(&buffer)?.split(&[' ', '\n'][..]).collect();

        // if the length is less than 3
        // we are going to return an error
        if contents.len() < 3 {
            return Err(FailedToParse);
        }

        let method = Method::try_from(*contents.get(0).unwrap())?;
        let route = contents.get(1).unwrap().to_string();
        let protocol = contents.get(2).unwrap().to_string();

        let request = Request::new(
            method,
            protocol,
            route,
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
        let buffer = "GET / HTTP/1.1\n".as_bytes();
        let request: Request = buffer.try_into().unwrap();

        assert_eq!("/", request.route);
        assert_eq!("HTTP/1.1", request.protocol);
        assert!(matches!(request.method, GET));
    }

    #[test]
    fn it_can_parse_other_request_methods() {
        let buffer = "POST / HTTP/1.1\nsomeHeader".as_bytes();
        let request: Request = buffer.try_into().unwrap();

        assert_eq!("/", request.route);
        assert_eq!("HTTP/1.1", request.protocol);
        assert!(matches!(request.method, POST));
    }

    #[test]
    fn it_throws_an_error_if_the_parsed_contents_are_not_splitted_into_at_least_three_chunks() {
        let buffer = "GET /".as_bytes();
        let request: Result<Request, RequestError> = buffer.try_into();

        match request {
            Ok(_) => panic!("This should have failed!"),
            Err(error) => assert!(matches!(error, RequestError::FailedToParse))
        }
    }

    #[test]
    fn it_throws_an_error_if_we_pass_an_invalid_http_request_method() {
        let buffer = "FAST / HTTP/1.1\nsomeHeader".as_bytes();
        let request: Result<Request, RequestError> = buffer.try_into();

        match request {
            Ok(_) => panic!("This should have failed!"),
            Err(error) => {
                match error {
                    RequestError::InvalidMethod(method) => assert_eq!("FAST", method.to_string()),
                    _ => panic!("This should have failed!")
                }
            }
        }
    }

    #[test]
    fn it_returns_an_error_if_it_cannot_be_encoded() {
        let buffer: &[u8] = &[255; 3][..];
        let request: Result<Request, RequestError> = buffer.try_into();

        match request {
            Ok(_) => panic!("This should have failed!"),
            Err(error) => assert!(matches!(error, RequestError::InvalidEncoding))
        }
    }
}
