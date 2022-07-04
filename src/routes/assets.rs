use crate::common::style::StyleSheet;
use rocket::{get, response::Redirect, routes, uri, Route, State};

#[get("/style.css")]
pub fn style() -> Redirect {
    Redirect::to(uri!(style_versioned(crate::VERSION)))
}

#[get("/<_version>/style.css")]
pub fn style_versioned<'a>(css: &'a State<StyleSheet>, _version: String) -> &'a StyleSheet<'a> {
    css
}

pub fn routes() -> Vec<Route> {
    routes![style, style_versioned]
}
