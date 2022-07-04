use crate::{common::ServerError, schema::User};
use rocket::{
    http::Status,
    outcome::{try_outcome, IntoOutcome, Outcome},
    request::{self, FromRequest, Request},
};

pub struct Driver();
pub struct DriverUser(User, Driver);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DriverUser {
    type Error = ServerError;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<User>().await);
        try_outcome!(user.is_admin().into_outcome(Status::Unauthorized));
        let admin_user = DriverUser(user, Driver());

        Outcome::Success(admin_user)
    }
}
