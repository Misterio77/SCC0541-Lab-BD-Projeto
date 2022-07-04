use crate::{
    common::ServerError,
    database::Database,
    schema::{Admin, Constructor, Driver, SpecializedUser, User, UserKind},
};
use rocket::{get, request::FlashMessage, response::Redirect, routes, uri, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/overview")]
pub fn overview(user: User) -> Redirect {
    Redirect::to(match user.kind {
        UserKind::Admin => uri!(overview_admin),
        UserKind::Driver => uri!(overview_driver),
        UserKind::Constructor => uri!(overview_constructor),
    })
}

#[get("/overview/admin")]
pub async fn overview_admin(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let admin = Admin::from_user(&db, &user).await?;
    let metrics = admin.get_metrics(&db).await?;
    let display_name = "Admin";

    Ok(Template::render(
        "overview-admin",
        context! {display_name,user,flash,metrics},
    ))
}

#[get("/overview/driver")]
pub async fn overview_driver(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    // let driver = Driver::from_user(&db, &user).await?;
    let display_name = "Driver X";
    Ok(Template::render(
        "overview-driver",
        context! {user,flash,display_name},
    ))
}

#[get("/overview/constructor")]
pub async fn overview_constructor(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    // let constructor = Constructor::from_user(&db, &user).await?;
    let display_name = "Constructor Y";
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
