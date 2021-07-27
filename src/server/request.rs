use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{Utf8Error};

use super::Method;
use crate::server::request::RequestError::{FailedToParse, InvalidMethod, InvalidProtocol};
use crate::server::MethodParseError;
use regex::Regex;
use crate::server::route::{Route, RouteError};

#[derive(Debug)]
pub enum RequestError {
    FailedToParse,
    InvalidEncoding,
    InvalidPath(String),
    InvalidMethod(String),
    InvalidProtocol(String),
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred {}", self.to_string())
    }
}

impl Error for RequestError {}

impl From<RouteError> for RequestError {
    fn from(_: RouteError) -> Self {
        RequestError::FailedToParse
    }
}

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

pub struct Request<'buf> {
    route: Route<'buf>,
    protocol: &'buf str,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn new(
        method: Method,
        protocol: &'buf str,
        route: Route<'buf>,
    ) -> Self {
        Request {
            route,
            method,
            protocol,
        }
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = RequestError;

    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        let contents: Vec<&str> = std::str::from_utf8(&buffer)?.split(&[' ', '\n'][..]).collect();

        // if the length is less than 3
        // we are going to return an error
        if contents.len() < 3 {
            return Err(FailedToParse);
        }

        let method = contents.get(0).unwrap().parse()?;
        let route = Route::new(contents.get(1).unwrap())?;
        let protocol = contents.get(2).unwrap();
        let protocol_regex = Regex::new(r"HTTP/(1\.1|2\.0)").unwrap();

        if !protocol_regex.is_match(protocol) {
            return Err(InvalidProtocol(protocol.to_string()));
        }

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

        assert_eq!("/", request.route.get_path());
        assert_eq!("HTTP/1.1", request.protocol);
        assert!(matches!(request.method, GET));
    }

    #[test]
    fn it_can_parse_other_request_methods() {
        let buffer = "POST / HTTP/1.1\nsomeHeader".as_bytes();
        let request: Request = buffer.try_into().unwrap();

        assert_eq!("/", request.route.get_path());
        assert_eq!("HTTP/1.1", request.protocol);
        assert!(matches!(request.method, POST));
    }

    #[test]
    fn it_supports_http_2() {
        let buffer = "POST / HTTP/2.0\nsomeHeader".as_bytes();
        let request: Request = buffer.try_into().unwrap();

        assert_eq!("/", request.route.get_path());
        assert_eq!("HTTP/2.0", request.protocol);
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

    #[test]
    fn it_throws_an_error_if_the_protocol_is_not_supported() {
        let buffer = "POST / HTTP/3.0\nsomeHeader".as_bytes();
        let request: Result<Request, RequestError> = buffer.try_into();

        match request {
            Ok(_) => panic!("This should have failed!"),
            Err(error) => {
                match error {
                    RequestError::InvalidProtocol(protocol) => assert_eq!("HTTP/3.0", protocol),
                    _ => panic!("This should have failed!")
                }
            }
        }
    }
}
