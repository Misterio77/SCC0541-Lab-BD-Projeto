use crate::{
    common::ServerError,
    database::Client,
    schema::{SpecializedUser, User, UserKind},
};
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Constructor(());

#[async_trait::async_trait]
impl SpecializedUser for Constructor {
    type Output = Constructor;
    async fn from_user(db: &Client, user: &User) -> Result<Constructor, ServerError> {
        if user.kind == UserKind::Constructor {
            Ok(Constructor(()))
        } else {
            Err(ServerError::forbidden())
        }
    }
}
