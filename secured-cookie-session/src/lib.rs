use actix_web::cookie::Cookie;
use async_trait::async_trait;

pub trait SessionRole: Eq
where
    Self: Sized,
{
}

pub trait SecuredSession
where
    Self: Sized,
{
    type Role: SessionRole;

    fn roles(&self) -> &[Self::Role];

    fn has_access(&self, assigned_roles: &[Self::Role]) -> bool {
        Self::is_roles_matching(self.roles(), assigned_roles)
    }

    fn is_roles_matching(current_roles: &[Self::Role], expected_roles: &[Self::Role]) -> bool {
        current_roles
            .iter()
            .any(|role| expected_roles.contains(role))
    }

    fn guest_cookie() -> Cookie<'static>;

    fn logined_cookie(&self) -> Cookie<'static>;
}

#[async_trait]
pub trait SessionService {
    type Role: SessionRole;
    type Session: SecuredSession<Role = Self::Role>;

    async fn session_by_id(&self, session_id: &str) -> Option<Self::Session>;
}
