use http::request::Request;
use http::response::Response;
pub use route_controller::RouteController;
pub use request_controller::RequestController;

mod route_controller;
mod request_controller;

pub trait Controller {
    fn handle(&self, request: &Request) -> Response;
}
