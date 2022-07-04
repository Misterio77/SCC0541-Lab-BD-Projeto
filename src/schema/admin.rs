use crate::{
    common::ServerError,
    database::Client,
    schema::{SpecializedUser, User, UserKind},
};
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Admin(());

/// Criar o admin
/// Pegamos a database mas não usamos, só pra manter a interface consistente
#[async_trait::async_trait]
impl SpecializedUser for Admin {
    type Output = Admin;
    async fn from_user(_db: &Client, user: &User) -> Result<Admin, ServerError> {
        if user.kind == UserKind::Admin {
            Ok(Admin(()))
        } else {
            Err(ServerError::forbidden())
        }
    }
}
