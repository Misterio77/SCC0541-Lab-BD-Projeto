use crate::{
    common::ServerError,
    database::Database,
    schema::{Admin, Constructor, Driver, User},
};
use rocket::{get, request::FlashMessage, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

// Essas são as funções roteamento para os relatórios.

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

    Ok(Template::render(
        "report2",
        context! {user,flash,report2,city},
    ))
}

#[get("/reports/3")]
pub async fn report3(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let constructor = Constructor::from_user(&db, &user).await?;
    let report3 = constructor.get_report3(&db).await?;

    Ok(Template::render("report3", context! {user,flash,report3}))
}

#[get("/reports/4")]
pub async fn report4(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let constructor = Constructor::from_user(&db, &user).await?;
    let report4 = constructor.get_report4(&db).await?;

    Ok(Template::render("report4", context! {user,flash,report4}))
}

#[get("/reports/5")]
pub async fn report5(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let driver = Driver::from_user(&db, &user).await?;
    let report5 = driver.get_report5(&db).await?;

    Ok(Template::render("report5", context! {user,flash,report5}))
}

#[get("/reports/6")]
pub async fn report6(
    user: User,
    flash: Option<FlashMessage<'_>>,
    db: Connection<Database>,
) -> Result<Template, ServerError> {
    let driver = Driver::from_user(&db, &user).await?;
    let report6 = driver.get_report6(&db).await?;

    Ok(Template::render("report6", context! {user,flash,report6}))
}

pub fn routes() -> Vec<Route> {
    routes![report1, report2, report3, report4, report5, report6]
}
