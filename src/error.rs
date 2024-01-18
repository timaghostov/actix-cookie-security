use std::error::Error as StdError;

use actix_web::HttpResponse;

pub type HttpResult = Result<HttpResponse, Box<dyn StdError>>;

pub struct WrapperHttpResult(pub HttpResult);

impl From<Result<HttpResponse, Box<dyn StdError>>> for WrapperHttpResult {
    fn from(result: Result<HttpResponse, Box<dyn StdError>>) -> Self {
        Self(result)
    }
}

impl From<HttpResponse> for WrapperHttpResult {
    fn from(response: HttpResponse) -> Self {
        WrapperHttpResult(Result::Ok(response))
    }
}