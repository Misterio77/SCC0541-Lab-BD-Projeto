use crate::common::style::StyleSheet;
use rocket::{get, response::Redirect, routes, Route, State};

#[get("/style.css")]
fn style() -> Redirect {
    Redirect::to(format!("/assets/{}/style.css", crate::VERSION))
}

#[get("/<_version>/style.css")]
fn style_versioned<'a>(css: &'a State<StyleSheet>, _version: String) -> &'a StyleSheet<'a> {
    css
}

pub fn routes() -> Vec<Route> {
    routes![style, style_versioned]
}
