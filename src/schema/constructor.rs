use crate::{
    common::ServerError,
    database::Client,
    schema::{SpecializedUser, User, UserKind},
};
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Constructor(());
