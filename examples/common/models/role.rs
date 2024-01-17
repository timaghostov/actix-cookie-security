use actix_cookie_security::SessionRole;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Role {
    Guest,
    Editor,
    Admin,
}

impl SessionRole for Role {}
