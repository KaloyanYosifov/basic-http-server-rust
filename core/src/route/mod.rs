use http::Method;
use crate::controller::Controller;
use http::request::Request;
use http::response::Response;

pub struct Route<'a> {
    method: Method,
    controller: Box<dyn Controller + 'a>,
}

impl<'a> Route<'a> {
    pub fn new(method: Method, controller: Box<dyn Controller>) -> Self {
        Self {
            method,
            controller,
        }
    }
}

impl<'a> Route<'a> {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn handle(&self, request: &Request) -> Response {
        self.controller.handle(&request)
    }
}
