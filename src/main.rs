use projeto_labbd::{
    common::{customize_tera, ServerError, StyleSheet},
    database::Database,
    routes::{account, assets, errors, home, overview, reports},
};

use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;

// Carregar a folha CSS em tempo de compilação pra dentro do binário
static STYLE: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    let rocket = rocket::build()
        // Middlewares (conexão com database e instância da template engine)
        .attach(Database::init())
        .attach(Template::custom(customize_tera))
        // Gerenciar a folha CSS pra ser servida
        .manage(StyleSheet::new(STYLE, 86400))
        // Rotas para apanhar erros
        .register("/", errors::catchers())
        // Conjuntos de rotas
        .mount("/assets", assets::routes())
        .mount("/", home::routes())
        .mount("/", account::routes())
        .mount("/", overview::routes())
        .mount("/", reports::routes());

    rocket.launch().await.ok();
    Ok(())
}
