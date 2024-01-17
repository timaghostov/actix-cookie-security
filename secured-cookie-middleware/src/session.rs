use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

use actix_web::cookie::Cookie;
use actix_web::dev::Extensions;
use actix_web::dev::Payload;
use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::HttpMessage;
use actix_web::HttpRequest;

use futures::future::ready;
use futures::future::Ready;

use secured_cookie_session::SecuredSession;
use secured_cookie_session::SessionRole;

struct SessionInner<SessionType> {
    session: Option<SessionType>,
}

pub struct Session<SessionType>(Rc<RefCell<SessionInner<SessionType>>>);

impl<SessionType> Default for SessionInner<SessionType> {
    fn default() -> Self {
        Self { session: None }
    }
}

impl<RoleType, SessionType> Session<SessionType>
where
    RoleType: SessionRole,
    SessionType: SecuredSession<Role = RoleType> + 'static,
{
    pub fn set_session(req: &ServiceRequest, session_object: Option<SessionType>) {
        let session = Session::get_session(&mut req.extensions_mut());
        let mut inner = session.0.borrow_mut();
        inner.session = session_object;
    }

    fn get_session(extensions: &mut Extensions) -> Self {
        if let Some(inner) = extensions.get::<Rc<RefCell<SessionInner<SessionType>>>>() {
            return Session(Rc::clone(inner));
        }
        let inner = Rc::new(RefCell::new(SessionInner::default()));
        extensions.insert(inner.clone());
        Session(inner)
    }

    pub fn is_authorized(&self) -> bool {
        self.0.borrow().session.is_some()
    }

    pub fn has_access(&self, assigned_roles: &[RoleType]) -> bool {
        match self.0.borrow().session.as_ref() {
            Some(session) => session.has_access(assigned_roles),
            None => false,
        }
    }

    pub fn cookie(&self) -> Cookie<'static> {
        match self.0.borrow().session.as_ref() {
            Some(session) => session.logined_cookie(),
            None => SessionType::guest_cookie(),
        }
    }

    pub fn as_inner(&self) -> Ref<'_, Option<SessionType>> {
        Ref::map(self.0.borrow(), |inner| &inner.session)
    }
}

impl<RoleType, SessionType> FromRequest for Session<SessionType>
where
    RoleType: SessionRole,
    SessionType: SecuredSession<Role = RoleType> + 'static,
{
    type Error = Error;
    type Future = Ready<Result<Session<SessionType>, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Ok(Session::get_session(&mut req.extensions_mut())))
    }
}
