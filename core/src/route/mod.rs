use http::Method;
use crate::controller::Controller;
use http::request::Request;
use http::response::Response;

pub struct Route {
    method: Method,
    controller: Box<dyn Controller>,
}

impl Route {
    pub fn new(method: Method, controller: Box<dyn Controller>) -> Self {
        Self {
            method,
            controller,
        }
    }
}

impl Route {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn handle(&self, request: &Request) -> Response {
        self.controller.handle(&request)
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

    struct BasicController;

    impl Controller for BasicController {
        fn handle(&self, _: &Request) -> Response {
            Response::new(StatusCode::NotFound, "".to_string())
        }
    }

    #[test]
    fn it_can_create_a_route() {
        let route = Route::new(Method::GET, Box::new(BasicController));
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
