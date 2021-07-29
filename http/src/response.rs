use std::io::Write;

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
pub struct Response {
    status: StatusCode,
    content: String,
}

impl Response {
    pub fn new(status: StatusCode, content: String) -> Self {
        Self {
            status,
            content,
        }
    }
}

impl Response {
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn send(&self, writer: &mut impl Write) -> std::io::Result<()> {
        write!(
            writer,
            "HTTP/1.1 {status} {status_message}\r\nContent-Length: {content_length}\r\n\r\n{content}",
            status = self.status as i32,
            status_message = self.status.name(),
            content_length = self.content.len(),
            content = self.content
        );
        writer.flush()?;

        Ok(())
    }
}
