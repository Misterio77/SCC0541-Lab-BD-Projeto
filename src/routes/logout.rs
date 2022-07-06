use crate::schema::User;
use rocket::{
    http::{Cookie, CookieJar},
    post,
    response::{Flash, Redirect},
    routes, uri, Route,
};

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>, _user: User) -> Result<Redirect, Flash<Redirect>> {
    cookies.remove_private(Cookie::named("user"));
    Ok(Redirect::to(uri!(super::home::home)))
}

pub fn routes() -> Vec<Route> {
    routes![logout]
}
