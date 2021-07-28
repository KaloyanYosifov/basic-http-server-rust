use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryParams<'buf> {
    params: HashMap<&'buf str, &'buf str>,
}

impl<'buf> QueryParams<'buf> {
    pub fn from_path(path: &'buf str) -> Self {
        let query_string_delimiter_index = path.find('?');

        if query_string_delimiter_index.is_none() {
            return Self {
                params: HashMap::new(),
            };
        }

        let query_string_delimiter_index = query_string_delimiter_index.unwrap() + 1;
        let query_slice = &path[query_string_delimiter_index..];
        let mut params: HashMap<&'buf str, &'buf str> = HashMap::new();

        fn split_param(param: &str) -> (&str, &str) {
            let splitted_param: Vec<&str> = param.split('=').collect();

            (splitted_param[0], splitted_param[1])
        }

        if query_slice.contains('&') {
            for param in query_slice.split('&') {
                let (name, value) = split_param(param);

                params.insert(name, value);
            }
        } else {
            let (name, value) = split_param(query_slice);

            params.insert(name, value);
        }

        Self {
            params,
        }
    }
}

impl<'buf> QueryParams<'buf> {
    pub fn get_param(&self, key: &str) -> Option<&&str> {
        self.params.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::query_params::QueryParams;

    #[test]
    fn it_can_parse_query_params_from_path() {
        let query_params = QueryParams::from_path(
            "/?hello=test&working=true"
        );

        assert!(query_params.is_not_empty());
        assert_eq!("test", *query_params.get_param("hello").unwrap());
        assert_eq!("true", *query_params.get_param("working").unwrap());
    }

    #[test]
    fn it_has_no_params_if_there_is_no_query_string() {
        let query_params = QueryParams::from_path(
            "/"
        );

        assert!(query_params.is_empty());
    }

    #[test]
    fn it_has_no_problem_to_parse_a_single_query_param() {
        let query_params = QueryParams::from_path(
            "/?hello=test"
        );

        assert!(query_params.is_not_empty());
        assert_eq!("test", *query_params.get_param("hello").unwrap());
    }
}
