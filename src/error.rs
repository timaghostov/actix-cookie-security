use std::fmt;

use actix_web::HttpResponse;
use actix_web::ResponseError;

#[derive(Debug)]
pub struct ApplicationError {
    error: Box<dyn ResponseError>,
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        self.error.error_response()
    }
}

impl ApplicationError {
    pub fn from<Error: ResponseError + 'static>(error: Error) -> Self {
        Self {
            error: Box::new(error),
        }
    }
}

pub type HttpResult = Result<HttpResponse, ApplicationError>;

#[derive(Debug)]
pub struct WrapperHttpResult(pub HttpResult);

impl<Error> From<Result<HttpResponse, Error>> for WrapperHttpResult
where
    Error: ResponseError + 'static,
{
    fn from(result: Result<HttpResponse, Error>) -> Self {
        WrapperHttpResult(result.map_err(ApplicationError::from))
    }
}

impl From<HttpResponse> for WrapperHttpResult {
    fn from(response: HttpResponse) -> Self {
        WrapperHttpResult(HttpResult::Ok(response))
    }
}
