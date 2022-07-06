/// Uma rota que lista as contas, pra conveniência
use crate::{
    common::ServerError,
    database::Database,
    schema::{User, UserKind},
};
use rocket::{get, request::FlashMessage, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/accounts")]
pub async fn accounts(
    user: Option<User>,
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, ServerError> {
    let users = User::list(&db).await?;

    // Isso é totalmente inseguro, claro. Mas não é como se a gente tivesse pensando nisso.
    let users: Vec<(String, User)> = users
        .into_iter()
        .filter_map(|user| {
            let password = match user.kind {
                UserKind::Admin => Some(user.login.as_str()),
                UserKind::Constructor => user.login.strip_suffix("_c"),
                UserKind::Driver => user.login.strip_suffix("_d"),
            };
            match password {
                Some(p) => Some((p.to_owned(), user)),
                None => None,
            }
        })
        .collect();

    Ok(Template::render("accounts", context! {flash, users, user}))
}

pub fn routes() -> Vec<Route> {
    routes![accounts]
}
