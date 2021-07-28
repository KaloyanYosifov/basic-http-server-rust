use crate::request::{Request, RequestError};
use crate::response::{Response, StatusCode};

pub trait RequestHandler {
    fn handle(&self, request: &Request) -> Response;
    fn handle_error(&self, request: &RequestError) -> Response {
        Response::new(StatusCode::BadRequest, "")
    }
}
