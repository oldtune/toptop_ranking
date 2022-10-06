use std::error::Error;

#[derive(Debug)]
pub enum HttpStatusCode {
    Success(u16),
    Failed(u16),
}

impl From<u16> for HttpStatusCode {
    fn from(code: u16) -> Self {
        match code {
            200..=299 => Self::Success(code),
            _ => Self::Failed(code),
        }
    }
}

#[derive(Debug)]
pub struct HttpError {
    response_code: Option<HttpStatusCode>,
    endpoint: String,
    err_content: String,
}

impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> Self {
        HttpError {
            response_code: err.status().map(|x| x.as_u16().into()),
            endpoint: err
                .url()
                .map(|x| x.to_string())
                .unwrap_or_else(|| String::new()),
            err_content: err.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum AppError {
    HttpError(HttpError),
    Unexpected(Box<dyn Error>),
}

pub type Result<T> = std::result::Result<T, AppError>;
