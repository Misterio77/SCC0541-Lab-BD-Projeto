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
        UserKind::Driver => panic!(),      // uri!(overview_driver),
        UserKind::Constructor => panic!(), // uri!(overview_constructor),
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
    let display_name = admin.display_name();

    Ok(Template::render(
        "overview-admin",
        context! {display_name,user,flash,metrics},
    ))
}

// TODO constructor, driver

pub fn routes() -> Vec<Route> {
    routes![overview, overview_admin]
}
