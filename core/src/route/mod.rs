use http::Method;
use crate::controller::Controller;

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
    pub fn get_method(&self) -> &Method {
        &self.method
    }
}
