use regex::Regex;
use crate::server::query_params::QueryParams;

pub enum RouteError {
    InvalidPath(String)
}

pub struct Route<'buf> {
    path: &'buf str,
    query_params: QueryParams<'buf>,
}

impl<'buf> Route<'buf> {
    pub fn new(path: &'buf str) -> Result<Self, RouteError> {
        if !Self::is_path_valid(&path) {
            return Err(RouteError::InvalidPath(path.to_string()));
        }

        let query_params = QueryParams::from_path(path);

        Ok(
            Self {
                path,
                query_params,
            }
        )
    }

    fn is_path_valid(path: &str) -> bool {
        let compiled_regex = Regex::new(r"^/([^/\s]+/?)?(.*)$").unwrap();

        compiled_regex.is_match(&path)
    }
}

impl<'buf> Route<'buf> {
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use crate::server::route::Route;
    use crate::server::route::RouteError;

    #[test]
    fn it_can_be_created_with_a_path() {
        let route = Route::new("/");

        assert!(true);
    }

    #[test]
    fn it_throws_an_error_if_the_path_doesnt_start_with_a_slash() {
        match Route::new("gangsta") {
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
