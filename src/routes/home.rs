use crate::schema::User;
use rocket::{get, response::Redirect, routes, uri, Route};

#[get("/")]
pub async fn home(user: Option<User>) -> Redirect {
    // Se estiver logado, ir pra dashboard. Se não, ir pro login
    Redirect::to(match user {
        Some(_) => uri!(super::overview::overview),
        None => uri!(super::login::login(_)),
    })
}

pub fn routes() -> Vec<Route> {
    routes![home]
}
