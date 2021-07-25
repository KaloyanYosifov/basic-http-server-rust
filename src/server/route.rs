pub enum RouteError {
    InvalidPath(String)
}

pub struct Route {
    path: String,
}

impl Route {
    pub fn new(path: String) -> Result<Self, RouteError> {
        if !Self::is_path_valid(&path) {
            return Err(RouteError::InvalidPath(path));
        }

        Ok(
            Self {
                path
            }
        )
    }

    fn is_path_valid(path: &str) -> bool {
        path.starts_with('/')
    }
}

impl Route {
    pub fn get_path(&self) -> &String {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use crate::server::route::Route;
    use crate::server::route::RouteError;

    #[test]
    fn it_can_be_created_with_a_path() {
        let route = Route::new("/".to_string());

        assert!(true);
    }

    #[test]
    fn it_throws_an_error_if_the_path_doesnt_start_with_a_slash() {
        match Route::new("gangsta".to_string()) {
            Err(error) => {
                match error {
                    RouteError::InvalidPath(path) => assert_eq!("gangsta", path),
                    _ => panic!("This should have failed!")
                }
            }
            _ => panic!("This should have failed!")
        }
    }
}
