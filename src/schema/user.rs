use crate::{
    common::ServerError,
    database::{Client, Database, Row},
};
use postgres_types::FromSql;
use rocket::{
    http::{Cookie, CookieJar, Status},
    outcome::{try_outcome, IntoOutcome},
    request::{self, FromRequest, Request},
};
use rocket_db_pools::Connection;
use serde::Serialize;

/// Representa um usuário do sistema
#[derive(Serialize, Debug)]
pub struct User {
    id: i32,
    pub login: String,
    pub kind: UserKind,
    original_id: Option<i32>,
}

/// Diferentes tipos de acesso
#[derive(Serialize, FromSql, PartialEq, Eq, Debug)]
#[postgres(name = "user_type")]
pub enum UserKind {
    #[postgres(name = "Administrador")]
    Admin,
    #[postgres(name = "Escuderia")]
    Constructor,
    #[postgres(name = "Piloto")]
    Driver,
}

impl User {
    // === Construtores ===
    /// Obtém um usuário dado seu ID. Função interna.
    async fn from_id(db: &Client, id: i32) -> Result<User, ServerError> {
        db.query_one(
            "SELECT userid, login, tipo, idoriginal
            FROM users
            WHERE userid = $1",
            &[&id],
        )
        .await?
        .try_into()
    }
    /// Obtém um usuário dado login e senha.
    pub async fn login(db: &Client, login: &str, password: &str) -> Result<User, ServerError> {
        db.query_one(
            "SELECT userid, login, tipo, idoriginal
            FROM users
            WHERE login = $1 AND password = md5($2)",
            &[&login, &password],
        )
        .await
        .map_err(|e| {
            // Adicionar mensagem amigável no erro genérico do postgres
            ServerError::builder_from(e)
                .message("Credenciais inválidas")
                .code(Status::Unauthorized)
                .build()
        })?
        .try_into()
    }
    /// Tentar extrair o id do usuário logado dos cookies (assinados pelo servidor, então são
    /// confiáveis)
    pub async fn authenticate(db: &Client, cookies: &CookieJar<'_>) -> Result<User, ServerError> {
        let cookie = cookies.get_private("user").ok_or_else(|| {
            ServerError::builder()
                .code(Status::Unauthorized)
                .message("Sessão inválida. Por favor, faça login novamente")
        })?;

        // Na verdade seria melhor se usássemos sessões ao invés de só guardar o ID, mas enfim, não
        // faz sentido num sistema de faz-de-conta.
        let id = cookie.value().parse::<i32>().unwrap();

        User::from_id(db, id).await
    }
}

/// Converter da schema da base para o nosso tipo aqui
impl TryFrom<Row> for User {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<User, ServerError> {
        Ok(User {
            id: row.try_get("userid")?,
            login: row.try_get("login")?,
            kind: row.try_get("tipo")?,
            original_id: row.try_get("idoriginal")?,
        })
    }
}

impl From<User> for Cookie<'_> {
    fn from(user: User) -> Self {
        Cookie::build("user", user.id.to_string())
            .permanent()
            .finish()
    }
}

/// Define uma interface para tipos de usuário que podem ser obtidos a partir do User
#[async_trait::async_trait]
pub trait SpecializedUser {
    type Output;
    /// Obter o usuário especializado a partir do genérico
    async fn from_user(db: &Client, user: &User) -> Result<Self::Output, ServerError>;
    type Metrics;
    /// Obter métricas que ele tem acesso
    async fn get_metrics(&self, db: &Client) -> Result<Self::Metrics, ServerError>;
    /// Obter o nome que ele deve ver
    fn display_name(&self) -> String;
}

/// Quando uma rota requisita User, esse traço vai rodar
/// Ele basicamente vai olhar os cookies da pessoa, verificar validade, e daí pegar o User na base
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ServerError;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Pegar os cookies
        let cookies = request.cookies();

        // Puxar conexão c/ a base do estado da aplicação
        let db = try_outcome!(request
            .guard::<Connection<Database>>()
            .await
            .map_failure(ServerError::from));

        let user = try_outcome!(User::authenticate(&db, cookies)
            .await
            .into_outcome(Status::Unauthorized));

        request::Outcome::Success(user)
    }
}
