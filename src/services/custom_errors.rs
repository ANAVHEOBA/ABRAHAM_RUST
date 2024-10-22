use std::fmt;

#[derive(Debug)]
pub struct BadRequest(pub String);

impl fmt::Display for BadRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bad Request: {}", self.0)
    }
}

#[derive(Debug)]
pub struct NotFound(pub String);

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not Found: {}", self.0)
    }
}

// Implementing std::error::Error for custom error types
impl std::error::Error for BadRequest {}
impl std::error::Error for NotFound {}
