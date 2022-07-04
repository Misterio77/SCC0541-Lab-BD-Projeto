use crate::{common::ServerError, schema::User};
use rocket::{
    http::Status,
    outcome::{try_outcome, IntoOutcome, Outcome},
    request::{self, FromRequest, Request},
};

pub struct Admin();
pub struct AdminUser(User, Admin);
impl AdminUser {
    pub fn into_user(self) -> User {
        self.0
    }
    pub fn as_user(&self) -> &User {
        &self.0
    }
    pub fn display_name(&self) -> String {
        "Admin".into()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ServerError;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<User>().await);
        try_outcome!(user.is_admin().into_outcome(Status::Unauthorized));
        let admin_user = AdminUser(user, Admin());

        Outcome::Success(admin_user)
    }
}
