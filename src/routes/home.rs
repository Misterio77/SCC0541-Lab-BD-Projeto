use crate::schema::User;
use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn home(user: Option<User>) -> Template {
    Template::render("base", context! {user})
}

pub fn routes() -> Vec<Route> {
    routes![home]
}
