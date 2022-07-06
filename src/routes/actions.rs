use crate::{
    common::ServerError,
    database::Database,
    schema::{Admin, Constructor, User},
};
use rocket::{
    form::{Form, FromForm},
    get, post,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, uri, Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::Deserialize;
use time::Date;

#[get("/actions/add-constructor")]
pub async fn add_constructor(
    db: Connection<Database>,
    user: User,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, ServerError> {
    // Verificar que é admin
    let _admin = Admin::from_user(&db, &user).await?;
    Ok(Template::render(
        "action-add-constructor",
        context! {user,flash},
    ))
}

#[post("/actions/add-constructor", data = "<form>")]
pub async fn add_constructor_submit(
    db: Connection<Database>,
    form: Form<AddConstructorForm>,
    user: User,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let admin = Admin::from_user(&db, &user)
        .await
        .map_err(|e| e.flash_redirect(uri!(add_constructor)))?;
    admin
        .add_constructor(&db, form.into_inner())
        .await
        .map_err(|e| e.flash_redirect(uri!(add_constructor)))?;

    Ok(Flash::success(
        Redirect::to(uri!(add_constructor)),
        "Inserção realizada com sucesso!",
    ))
}

#[derive(FromForm, Deserialize)]
pub struct AddConstructorForm {
    pub reference: String,
    pub name: String,
    pub nationality: String,
    pub url: Option<String>,
}

#[get("/actions/add-driver")]
pub async fn add_driver(
    db: Connection<Database>,
    user: User,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, ServerError> {
    // Verificar que é admin
    let _admin = Admin::from_user(&db, &user).await?;
    Ok(Template::render("action-add-driver", context! {user,flash}))
}

#[post("/actions/add-driver", data = "<form>")]
pub async fn add_driver_submit(
    db: Connection<Database>,
    form: Form<AddDriverForm>,
    user: User,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let admin = Admin::from_user(&db, &user)
        .await
        .map_err(|e| e.flash_redirect(uri!(add_driver)))?;
    admin
        .add_driver(&db, form.into_inner())
        .await
        .map_err(|e| e.flash_redirect(uri!(add_driver)))?;

    Ok(Flash::success(
        Redirect::to(uri!(add_driver)),
        "Inserção realizada com sucesso!",
    ))
}

#[derive(FromForm, Deserialize)]
pub struct AddDriverForm {
    pub reference: String,
    pub forename: String,
    pub surname: String,
    pub dob: Date,
    pub nationality: String,
    pub number: Option<i32>,
    pub code: Option<String>,
    pub url: Option<String>,
}

#[get("/actions/get-driver?<forename>")]
pub async fn get_driver(
    db: Connection<Database>,
    user: User,
    flash: Option<FlashMessage<'_>>,
    forename: Option<String>,
) -> Result<Template, ServerError> {
    // Verificar que é escuderia
    let constructor = Constructor::from_user(&db, &user).await?;

    let forename = forename.unwrap_or_default();
    let drivers = constructor.drivers_by_forename(&db, &forename).await?;
    Ok(Template::render(
        "action-get-driver",
        context! {user,flash,forename,drivers},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![
        add_constructor,
        add_constructor_submit,
        add_driver,
        add_driver_submit,
        get_driver
    ]
}
