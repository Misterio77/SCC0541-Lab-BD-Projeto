use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn home() -> Template {
    Template::render("base", context! {})
}

pub fn routes() -> Vec<Route> {
    routes![home]
}
