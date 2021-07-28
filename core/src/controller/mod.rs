use http::request::Request;
use http::response::Response;

pub trait Controller {
    fn handle(&self, request: &Request) -> Response;
}
