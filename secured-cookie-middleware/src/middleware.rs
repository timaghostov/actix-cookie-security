use std::fmt::Display;
use std::marker::PhantomData;
use std::rc::Rc;

use actix_web::dev::forward_ready;
use actix_web::dev::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::dev::Transform;
use actix_web::error::Error;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::web;

use futures::future::ready;
use futures::future::LocalBoxFuture;
use futures::future::Ready;

use secured_cookie_session::SecuredSession;
use secured_cookie_session::SessionRole;
use secured_cookie_session::SessionService;

use crate::session;
use crate::util;

pub struct SessionMiddleware<AppContext> {
    session_cookie_key: Rc<String>,
    application_context: PhantomData<AppContext>,
}

impl<RoleType, SessionType, AppContext> SessionMiddleware<AppContext>
where
    AppContext: SessionService<Role = RoleType, Session = SessionType>,
{
    #[allow(dead_code)]
    pub fn new(session_cookie_key: impl Display) -> Self {
        Self {
            session_cookie_key: Rc::new(session_cookie_key.to_string()),
            application_context: PhantomData::<AppContext>,
        }
    }
}

pub struct InnerSessionMiddleware<ActixService, AppContext> {
    service: Rc<ActixService>,
    session_cookie_key: Rc<String>,
    application_context: PhantomData<AppContext>,
}

impl<ActixService, B, RoleType, SessionType, AppContext> Transform<ActixService, ServiceRequest>
    for SessionMiddleware<AppContext>
where
    ActixService: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    ActixService::Future: 'static,
    B: 'static,
    RoleType: SessionRole,
    SessionType: SecuredSession<Role = RoleType> + 'static,
    AppContext: SessionService<Role = RoleType, Session = SessionType> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = InnerSessionMiddleware<ActixService, AppContext>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: ActixService) -> Self::Future {
        ready(Ok(InnerSessionMiddleware {
            service: Rc::new(service),
            session_cookie_key: self.session_cookie_key.clone(),
            application_context: PhantomData::<AppContext>,
        }))
    }
}

impl<ActixService, B, RoleType, SessionType, AppContext> Service<ServiceRequest>
    for InnerSessionMiddleware<ActixService, AppContext>
where
    ActixService: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    ActixService::Future: 'static,
    B: 'static,
    RoleType: SessionRole,
    SessionType: SecuredSession<Role = RoleType> + 'static,
    AppContext: SessionService<Role = RoleType, Session = SessionType> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let cookie_name = self.session_cookie_key.clone();

        Box::pin(async move {
            let session_id = util::filter_cookie(&req, &cookie_name);
            let application = req
                .app_data::<web::Data<AppContext>>()
                .ok_or(InternalError::new(
                    "Unknown application context",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))?;

            let session = match session_id.as_deref() {
                Some(session_id) => application.get_ref().session_by_id(session_id).await,
                None => None,
            };

            session::Session::set_session(&req, session);

            let result = service.call(req).await?;

            Ok(result)
        })
    }
}
