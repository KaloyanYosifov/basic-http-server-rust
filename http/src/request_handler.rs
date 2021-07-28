use crate::request::Request;
use crate::response::Response;

pub trait RequestHandler {
    fn handle(&self, request: &Request) -> Response;
}
