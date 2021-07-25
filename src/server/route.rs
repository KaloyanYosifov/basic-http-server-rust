pub struct Route {
    path: String,
}

impl Route {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }
}

impl Route {
    pub fn get_path(&self) -> &String {
        &self.path
    }
}
