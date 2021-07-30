use http::Method;
use crate::controller::{Controller, RequestController};
use http::request::Request;
use http::response::Response;

pub struct Route {
    path: String,
    method: Method,
    controller: Box<dyn Controller>,
}

impl Route {
    pub fn new(path: String, method: Method, controller: Box<dyn Controller>) -> Self {
        Self {
            path,
            method,
            controller,
        }
    }
}

impl Route {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn handle(&self, request: &Request, request_controller: &RequestController) -> Response {
        self.controller.handle(&request, &request_controller)
    }
}

#[cfg(test)]
mod tests {
    use crate::controller::Controller;
    use http::request::Request;
    use http::response::{Response, StatusCode};
    use crate::route::Route;
    use http::route::Route as HttpRoute;
    use http::Method;

    #[test]
    fn it_can_create_a_route() {
        let route = Route::new(
            "/".to_string(),
            Method::GET,
            Box::new(crate::controller::FakeController::new()),
        );
        let request = Request::new(
            Method::GET,
            "HTTP/1.1",
            HttpRoute::new("/").unwrap(),
        );
        let response = route.controller.handle(&request);
        assert!(matches!(route.method, Method::GET));
        assert!(matches!(response.status(), StatusCode::NotFound));
    }
}
