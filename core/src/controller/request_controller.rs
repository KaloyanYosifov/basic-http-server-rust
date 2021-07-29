use http::request_handler::RequestHandler;
use http::request::Request;
use http::response::{Response, StatusCode};
use std::path::PathBuf;
use crate::route::Route;
use crate::controller::RouteController;

pub struct RequestController<'rc> {
    path: PathBuf,
    route_controller: &'rc RouteController,
}

impl<'rc> RequestController<'rc> {
    pub fn new(path: String, route_controller: &'rc RouteController) -> Self {
        let canonicalized_path = std::fs::canonicalize(&path).unwrap();

        println!("Public path is: {}", canonicalized_path.to_str().unwrap());

        Self {
            route_controller,
            path: canonicalized_path,
        }
    }
}

impl<'rc> RequestController<'rc> {
    fn file_to_response(&self, path: &str) -> Response {
        let real_path = match std::fs::canonicalize(
            format!("{}{}", self.path.to_str().unwrap(), path)
        ) {
            Ok(path) => path,
            Err(_) => return Response::new(StatusCode::NotFound, "".to_string()),
        };

        if !real_path.starts_with(&self.path) {
            println!("Malicious: {}", real_path.to_str().unwrap());

            return Response::new(StatusCode::NotFound, "".to_string());
        }

        println!("{}", real_path.to_str().unwrap());
        match std::fs::read_to_string(std::path::Path::new(real_path.as_path())) {
            Ok(contents) => Response::new(StatusCode::OK, contents),
            Err(_) => Response::new(StatusCode::NotFound, "".to_string()),
        }
    }
}

impl<'rc> RequestHandler for RequestController<'rc> {
    fn handle(&self, request: &Request) -> Response {
        let index = self.route_controller
            .routes()
            .iter()
            .position(|route| route.path() == request.route().get_path());

        if index.is_some() {
            self.route_controller.routes().get(index.unwrap()).unwrap().handle(&request)
        } else {
            Response::new(StatusCode::NotFound, "".to_string())
        }
    }
}
