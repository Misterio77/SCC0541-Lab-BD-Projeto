use crate::common::ServerError;
use rocket::{
    catch, catchers,
    http::Status,
    response::{Flash, Redirect},
    Catcher, Request,
};

#[catch(401)]
pub fn unauthorized() -> Flash<Redirect> {
    let error = ServerError::builder()
        .code(Status::Unauthorized)
        .message("Faça login antes")
        .build();

    error.flash_redirect("/login")
}

#[catch(403)]
pub fn forbidden() -> ServerError {
    ServerError::builder()
        .code(Status::Forbidden)
        .message("Sua conta não tem acesso a essa página. Talvez experimente usar outra?")
        .build()
}

#[catch(404)]
pub fn not_found() -> ServerError {
    ServerError::builder()
        .code(Status::NotFound)
        .message("Página não encontrada")
        .build()
}

#[catch(503)]
pub fn service_unavailable() -> ServerError {
    ServerError::builder()
        .code(Status::ServiceUnavailable)
        .message("Serviço temporariamente indisponível")
        .build()
}

#[catch(default)]
pub fn unknown_error(status: Status, _: &Request) -> ServerError {
    ServerError::builder()
        .code(status)
        .message("Erro inesperado :(")
        .build()
}

pub fn catchers() -> Vec<Catcher> {
    catchers![
        not_found,
        service_unavailable,
        unknown_error,
        unauthorized,
        forbidden
    ]
}
