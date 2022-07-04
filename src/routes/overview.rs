use crate::{
    common::ServerError,
    schema::{User, UserKind},
};
use rocket::{get, request::FlashMessage, response::Redirect, routes, uri, Route};
use rocket_dyn_templates::{context, Template};

#[get("/overview")]
pub fn overview(user: User) -> Redirect {
    Redirect::to(match user.inner {
        UserKind::Admin(_) => uri!(overview_admin),
        UserKind::Driver(_) => uri!(overview_driver),
        UserKind::Constructor(_) => uri!(overview_constructor),
    })
}

#[get("/overview/admin")]
pub async fn overview_admin(
    user: User,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, ServerError> {
    user.is_admin()?;
    Ok(Template::render(
        "overview-admin",
        context! {
            display_name: user.display_name(),
            user,
            flash,
        },
    ))
}

#[get("/overview/driver")]
pub async fn overview_driver(
    user: User,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, ServerError> {
    // Verificar que o usuário é motorista
    user.is_driver()?;

    let display_name = "Motorista X";

    Ok(Template::render(
        "overview-driver",
        context! {user,flash,display_name},
    ))
}

#[get("/overview/constructor")]
pub async fn overview_constructor(
    user: User,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, ServerError> {
    // Verificar que o usuário é escuderia
    user.is_constructor()?;

    let display_name = "Construtora X";

    Ok(Template::render(
        "overview-constructor",
        context! {user,flash,display_name},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![
        overview,
        overview_admin,
        overview_driver,
        overview_constructor
    ]
}
