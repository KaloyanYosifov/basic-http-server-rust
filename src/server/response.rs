#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    OK = 200,
    NotFound = 404,
    BadRequest = 400,
    ServerError = 500,
}

impl StatusCode {
    pub fn name(&self) -> &str {
        match self {
            StatusCode::OK => "OK",
            StatusCode::NotFound => "Not Found",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::ServerError => "Server Error",
            _ => "Unknown"
        }
    }
}

#[derive(Debug)]
pub struct Response<'content> {
    status: StatusCode,
    content: &'content str,
}

impl<'content> Response<'content> {
    pub fn new(status: StatusCode, content: &'content str) -> Self {
        Self {
            status,
            content,
        }
    }
}

impl<'content> Response<'content> {
    pub fn get_status_code(&self) -> &StatusCode {
        &self.status
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn stringify(&self) -> String {
        let response = format!(
            "HTTP/1.1 {status} {status_message}\r\nContent-Length: {content_length}\r\n\r\n{content}",
            status = self.status as i32,
            status_message = self.status.name(),
            content_length = self.content.len(),
            content = self.content
        );

        response
    }
}