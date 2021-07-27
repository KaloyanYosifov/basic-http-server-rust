use std::collections::HashMap;

pub struct QueryParams {
    params: HashMap<String, String>,
}

impl QueryParams {
    pub fn new(query: String) -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn from_path(path: String) -> Self {
        let query_string_delimiter_index = path.find('?');

        if query_string_delimiter_index.is_none() {
            return Self {
                params: HashMap::new(),
            };
        }

        let query_string_delimiter_index = query_string_delimiter_index.unwrap() + 1;
        let query_slice = &path[query_string_delimiter_index..];
        let mut params: HashMap<String, String> = HashMap::new();

        if query_slice.contains('&') {
            for param in query_slice.split('&') {
                let splitted_param: Vec<&str> = param.split('=').collect();
                let name = splitted_param[0];
                let value = splitted_param[1];

                params.insert(name.to_string(), value.to_string());
            }
        } else {
            let splitted_param: Vec<&str> = query_slice.split('=').collect();
            let name = splitted_param[0];
            let value = splitted_param[1];

            params.insert(name.to_string(), value.to_string());
        }

        Self {
            params,
        }
    }
}

impl QueryParams {
    fn get_param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
    fn is_empty(&self) -> bool {
        self.params.is_empty()
    }
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::server::query_params::QueryParams;

    #[test]
    fn it_can_parse_query_params_from_path() {
        let query_params = QueryParams::from_path(
            "/?hello=test&working=true".to_string()
        );

        assert!(query_params.is_not_empty());
        assert_eq!("test", query_params.get_param("hello").unwrap());
        assert_eq!("true", query_params.get_param("working").unwrap());
    }

    #[test]
    fn it_has_no_params_if_there_is_no_query_string() {
        let query_params = QueryParams::from_path(
            "/".to_string()
        );

        assert!(query_params.is_empty());
    }

    #[test]
    fn it_has_no_problem_to_parse_a_single_query_param() {
        let query_params = QueryParams::from_path(
            "/?hello=test".to_string()
        );

        assert!(query_params.is_not_empty());
        assert_eq!("test", query_params.get_param("hello").unwrap());
    }
}
