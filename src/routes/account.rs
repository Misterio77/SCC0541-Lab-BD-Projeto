use crate::{common::ServerError, database::Database, schema::User};
use rocket::{
    form::{Form, FromForm},
    get,
    http::{Cookie, CookieJar},
    post,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, uri, Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::Deserialize;

/// Rota GET da tela de login
#[get("/login?<prev>")]
pub fn login_screen(
    prev: Option<String>, // Esse cara é o ultimo login que foi usado, pra pré-preencher o campo
    user: Option<User>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Flash<Redirect>> {
    // Se estiver logado, redirecionar
    if user.is_some() {
        Err(ServerError::builder()
            .message("Você já está logado")
            .build()
            .flash_redirect("/"))
    // Se não, mostrar a tela de login
    } else {
        Ok(Template::render("login", context! {flash, prev}))
    }
}

/// Rota POST da submissão do formulário de login
#[post("/login", data = "<form>")]
pub async fn login_submit(
    db: Connection<Database>,
    form: Form<LoginForm>,
    cookies: &CookieJar<'_>,
    user: Option<User>,
) -> Result<Redirect, Flash<Redirect>> {
    // Se já estiver logado, redirecionar
    if user.is_some() {
        return Err(ServerError::builder()
            .message("Você já está logado")
            .build()
            .flash_redirect("/"));
    };

    // Desestruturar a struct (haha)
    let LoginForm { login, password } = form.into_inner();

    // Fazer o login
    let logged_user = User::login(&db, &login, &password)
        .await
        // Se der ruim, redirecionar pro login com a tentativa atual como login pré-preenchido
        .map_err(|e| e.flash_redirect(&format!("/login?prev={}", &login)))?;

    // Se deu tudo certo, adicionar sessão nos cookies e ir pra home
    cookies.add_private(logged_user.into());
    Ok(Redirect::to(uri!(super::overview::overview)))
}
/// Representação do formulário de login
#[derive(FromForm, Deserialize)]
pub struct LoginForm {
    login: String,
    password: String,
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>, _user: User) -> Result<Redirect, Flash<Redirect>> {
    cookies.remove_private(Cookie::named("user"));
    Ok(Redirect::to(uri!(super::overview::overview)))
}

pub fn routes() -> Vec<Route> {
    routes![login_screen, login_submit, logout]
}
