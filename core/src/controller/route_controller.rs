use crate::route::Route;

pub struct RouteController {
    routes: Vec<Route>,
}

impl RouteController {
    pub fn new() -> Self {
        Self {
            routes: vec![]
        }
    }
}

impl RouteController {
    pub fn add(&mut self, route: Route) -> &mut Self {
        self.routes.push(route);

        self
    }

    pub fn routes(&self) -> &Vec<Route> {
        &self.routes
    }
}

#[cfg(test)]
mod tests {
    use crate::controller::{RouteController, FakeController};
    use crate::route::Route;
    use http::Method;

    #[test]
    fn it_can_add_routes() {
        let mut route_controller = RouteController::new();
        route_controller
            .add(
                Route::new(
                    "/".to_string(),
                    Method::POST,
                    Box::new(FakeController::new()),
                )
            );

        let first_route = route_controller.routes.get(0).unwrap();
        assert_eq!("/", first_route.path());
        assert!(matches!(first_route.method(), Method::POST));
    }
}
