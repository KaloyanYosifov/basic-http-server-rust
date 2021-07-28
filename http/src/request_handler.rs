use crate::request::{Request, RequestError};
use crate::response::Response;

pub trait RequestHandler {
    fn handle(&self, request: &Request) -> Response;
    fn handle_error(&self, request: &RequestError) -> Response;
}
