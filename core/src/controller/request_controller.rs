use http::request_handler::RequestHandler;
use http::request::Request;
use http::response::{Response, StatusCode};

pub struct RequestController;

impl RequestHandler for RequestController {
    fn handle(&self, request: &Request) -> Response {
        match request.route().get_path() {
            "/" => Response::new(StatusCode::OK, "<html><h1>Hello World</h1></html>"),
            "/welcome" => Response::new(StatusCode::OK, "<html><h1>Hello Welcome</h1></html>"),
            _ => Response::new(StatusCode::NotFound, ""),
        }
    }
}
