use crate::{
    common::ServerError,
    database::Database,
    schema::{Admin, Constructor, Driver, User, UserKind},
};
use rocket::tokio::try_join;
use rocket::{get, request::FlashMessage, response::Redirect, routes, uri, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/overview")]
pub fn overview(user: User) -> Redirect {
    Redirect::to(match user.kind {
        UserKind::Admin => uri!(overview_admin),
        UserKind::Driver => uri!(overview_driver), // uri!(overview_driver),
        UserKind::Constructor => uri!(overview_constructor), // uri!(overview_constructor),
    })
}

#[get("/overview/admin")]
pub async fn overview_admin(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let (admin, last_login) = try_join!(Admin::from_user(&db, &user), user.get_last_login(&db))?;
    let metrics = admin.get_metrics(&db).await?;
    let display_name = admin.display_name();

    Ok(Template::render(
        "overview-admin",
        context! {display_name,user,flash,metrics,last_login},
    ))
}

#[get("/overview/constructor")]
pub async fn overview_constructor(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let (constructor, last_login) =
        try_join!(Constructor::from_user(&db, &user), user.get_last_login(&db))?;
    let metrics = constructor.get_metrics(&db).await?;
    let display_name = constructor.display_name();

    Ok(Template::render(
        "overview-constructor",
        context! {display_name,user,flash,metrics,last_login},
    ))
}

#[get("/overview/driver")]
pub async fn overview_driver(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let (driver, last_login) = try_join!(Driver::from_user(&db, &user), user.get_last_login(&db))?;
    let metrics = driver.get_metrics(&db).await?;
    let display_name = driver.display_name();

    Ok(Template::render(
        "overview-driver",
        context! {display_name,user,flash,metrics,last_login},
    ))
}

// Sim, as funções são quase idênticas.
// Eu queria fazer algum tipo de polimorfismo ou macro pra deixar mais DRY, mas o tempo tá muito
// curto.

pub fn routes() -> Vec<Route> {
    routes![
        overview,
        overview_admin,
        overview_constructor,
        overview_driver
    ]
}
