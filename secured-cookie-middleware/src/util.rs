use actix_web::dev::ServiceRequest;

pub fn filter_cookie(req: &ServiceRequest, cookie_name: &str) -> Option<String> {
    let value = req.cookie(cookie_name).map(|c| c.value().to_owned());
    match value {
        None => None,
        Some(cookie) if cookie.is_empty() => None,
        Some(cookie) => Some(cookie),
    }
}
