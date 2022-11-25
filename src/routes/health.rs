use rocket::{get, routes, Route};

#[get("/healthy")]
pub fn healthy() {}

#[get("/ready")]
pub fn ready() {}

pub fn routes() -> Vec<Route> {
    routes![healthy, ready]
}
