use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    InvalidInput,
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::InvalidInput => write!(f, "Invalid input provided"),
            AppError::InternalError(ref err) => write!(f, "Internal error: {}", err),
        }
    }
}

impl AppError {
    pub fn internal_error(msg: &str) -> Self {
        AppError::InternalError(msg.to_string())
    }
}
