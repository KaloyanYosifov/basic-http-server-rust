use http::request::Request;
use http::response::{Response, StatusCode};
use crate::controller::{Controller, RequestController};

type HandleFunction = fn(request: &Request) -> Response;

pub struct FakeController {
    handle: Option<HandleFunction>,
}

impl FakeController {
    pub fn new() -> Self {
        Self {
            handle: None
        }
    }

    pub fn new_with_handle(handle: HandleFunction) -> Self {
        Self {
            handle: Some(handle)
        }
    }
}

impl Controller for FakeController {
    fn handle(&self, request: &Request, _: &RequestController) -> Response {
        if let Some(handle) = self.handle {
            handle(&request)
        } else {
            Response::new(StatusCode::NotFound, "".to_string())
        }
    }
}
