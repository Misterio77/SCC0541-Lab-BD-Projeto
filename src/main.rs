use projeto_labbd::{common::style::StyleSheet, database::Database, routes::assets};

use anyhow::Result;

use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;

// Carregar a folha CSS em tempo de compilação pra dentro do binário
static STYLE: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

#[rocket::main]
async fn main() -> Result<()> {
    let rocket = rocket::build()
        // Middlewares (conexão com database e template engine)
        .attach(Database::init())
        .attach(Template::fairing())
        // Gerenciar a folha CSS pra ser servida
        .manage(StyleSheet::new(STYLE, 86400))
        // Rotas
        .mount("/assets", assets::routes());

    rocket.launch().await.ok();
    Ok(())
}
