use http::request::Request;
use http::response::Response;
pub use fake_controller::FakeController;
pub use route_controller::RouteController;
pub use request_controller::RequestController;

mod fake_controller;
mod route_controller;
mod request_controller;

pub trait Controller {
    fn handle(&self, request: &Request, request_controller: &RequestController) -> Response;
}
