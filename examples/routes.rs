use actix_cookie_security::{secured, SecuredSession};
use actix_web::{get, post, web, HttpResponse};

use crate::common::app::Application;
use crate::common::error::ApiError;
use crate::common::models::Role;
use crate::common::models::SessionAggregate;

type Session = actix_cookie_security::Session<SessionAggregate>;

#[post("/login")]
async fn login(app: web::Data<Application>) -> Result<HttpResponse, ApiError> {
    let new_session = app.session_service.login().await?;

    Ok(HttpResponse::Ok()
        .cookie(new_session.logined_cookie())
        .finish())
}

#[get("/guest_handle")]
async fn guest_handle() -> HttpResponse {
    HttpResponse::Ok().body("guest_handle")
}

#[secured(session, [Role::Editor])]
#[get("/editor_handle")]
async fn editor_handle(
    _app: web::Data<Application>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("editor_handle"))
}

#[secured(session, [Role::Admin])]
#[get("/admin_handle")]
async fn admin_handle(session: Session) -> HttpResponse {
    HttpResponse::Ok().body("admin_handle 2")
}

fn custom_unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().finish()
}

fn custom_forbidden() -> HttpResponse {
    HttpResponse::Forbidden().finish()
}

#[secured(
    session,
    [Role::Editor, Role::Admin],
    unauthorized_function = custom_unauthorized,
    forbidden_function = custom_forbidden
)]
#[get("/editor_admin_handle")]
async fn editor_admin_handle(
    _app: web::Data<Application>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("editor_admin_handle"))
}

#[get("/logout")]
async fn logout(app: web::Data<Application>, session: Session) -> Result<HttpResponse, ApiError> {
    let session_id = session
        .as_inner()
        .as_ref()
        .map(|s| s.session.session_id)
        .ok_or(ApiError::Unauthorized)?;
    app.session_service.logout(session_id).await?;

    Ok(HttpResponse::Ok()
        .cookie(SessionAggregate::guest_cookie())
        .finish())
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(guest_handle)
        .service(editor_handle)
        .service(admin_handle)
        .service(editor_admin_handle)
        .service(logout);
}

#[derive(Debug)]
struct ExampleCustomError;
