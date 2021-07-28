use http::request::Request;
use http::response::Response;
pub use request_controller::RequestController;

mod request_controller;

pub trait Controller {
    fn handle(&self, request: &Request) -> Response;
}
