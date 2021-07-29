use http::request_handler::RequestHandler;
use http::request::Request;
use http::response::{Response, StatusCode};

pub struct RequestController {
    path: String,
}

impl RequestController {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }
}

impl RequestController {
    fn file_to_response(&self, path: &str) -> Response {
        let real_path = std::fs::canonicalize(
            format!("{}/{}", self.path, path)
        )
            .unwrap();

        if !real_path.starts_with(&self.path) {
            println!("{}", real_path.to_str().unwrap());
            return Response::new(StatusCode::NotFound, "".to_string());
        }

        println!("{}", real_path.to_str().unwrap());
        match std::fs::read_to_string(std::path::Path::new(real_path.as_path())) {
            Ok(contents) => Response::new(StatusCode::OK, contents),
            Err(_) => Response::new(StatusCode::NotFound, "".to_string()),
        }
    }
}

impl RequestHandler for RequestController {
    fn handle(&self, request: &Request) -> Response {
        match request.route().get_path() {
            "/" => self.file_to_response("/index.html"),
            "/welcome" => Response::new(StatusCode::OK, "<html><h1>Hello Welcome</h1></html>".to_string()),
            path => self.file_to_response(path)
        }
    }
}
