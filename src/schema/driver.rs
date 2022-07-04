use crate::{
    common::ServerError,
    database::Client,
    schema::{SpecializedUser, User, UserKind},
};
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Driver(());

#[async_trait::async_trait]
impl SpecializedUser for Driver {
    type Output = Driver;
    async fn from_user(db: &Client, user: &User) -> Result<Driver, ServerError> {
        if user.kind == UserKind::Driver {
            Ok(Driver(()))
        } else {
            Err(ServerError::forbidden())
        }
    }
}
