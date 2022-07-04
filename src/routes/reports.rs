use crate::{
    common::ServerError,
    database::Database,
    schema::{Admin, SpecializedUser, User},
};
use rocket::{get, request::FlashMessage, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/reports/1")]
pub async fn report1(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let admin = Admin::from_user(&db, &user).await?;
    let report1 = admin.get_report1(&db).await?;

    Ok(Template::render("report1", context! {user,flash,report1}))
}

#[get("/reports/2?<city>")]
pub async fn report2(
    user: User,
    city: Option<String>,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let admin = Admin::from_user(&db, &user).await?;

    // Se não especificar a cidade, vazio
    let city = city.unwrap_or_default();
    let report2 = admin.get_report2(&db, &city).await?;

    Ok(Template::render("report2", context! {user,flash,report2,city}))
}

pub fn routes() -> Vec<Route> {
    routes![report1, report2]
}
