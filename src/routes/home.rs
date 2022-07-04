use crate::schema::User;
use rocket::{uri, get, response::Redirect, routes, Route};

#[get("/")]
pub async fn home(user: Option<User>) -> Redirect {
    // Se estiver logado, ir pra dashboard. Se não, ir pro login
    Redirect::to(match user {
        Some(_) => uri!(super::overview::overview),
        None => uri!(super::account::login_screen(_)),
    })
}

pub fn routes() -> Vec<Route> {
    routes![home]
}
