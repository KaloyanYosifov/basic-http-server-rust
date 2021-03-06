use core::controller::{Controller, RequestController};
use http::request::Request;
use http::response::Response;
use http::Method::GET;

pub struct HomeController;

impl Controller for HomeController {
    #[core_macros::route("/", GET)]
    fn handle(&self, request: &Request, request_controller: &RequestController) -> Response {
        request_controller.file_to_response("/index.html")
    }
}
