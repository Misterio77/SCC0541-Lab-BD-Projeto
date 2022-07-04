use crate::{
    common::ServerError,
    database::{Client, Database, Row},
    schema::{Admin, Constructor, Driver},
};
use rocket::{
    http::{Cookie, CookieJar, Status},
    outcome::{try_outcome, IntoOutcome},
    request::{self, FromRequest, Request},
};
use rocket_db_pools::Connection;
use postgres_types::FromSql;
use serde::Serialize;

/// Representa um usuário do sistema
#[derive(Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub inner: UserKind,
}

/// Diferentes tipos de acesso
///
/// Esse enum tem tipos concretos dentro das variantes,
/// isso cria uma espécie de polimorfismo.
///
/// Então, basicamente, cada User tem coisas em comum (id, login), mas também tem um tipo
/// especializado dentro (admin, constructor, ou driver).
#[derive(Serialize, PartialEq, Eq, Debug)]
pub enum UserKind {
    Admin(Admin),
    Constructor(Constructor),
    Driver(Driver),
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

    // === Métodos ===
    /// Exibe o nome de exibição, com lógica diferente pra cada variante
    pub fn display_name(&self) -> String {
        match self.inner {
            UserKind::Admin(_) => "Admin",
            UserKind::Constructor(_) => "Constructor", // TODO
            UserKind::Driver(_) => "Driver",           // TODO
        }
        .into()
    }
    /// Retorna um erro se não for admin
    pub fn is_admin(&self) -> Result<(), ServerError> {
        authorized_to_result(matches!(self.inner, UserKind::Admin(_)))
    }
    /// Retorna um erro se não for constructor
    pub fn is_constructor(&self) -> Result<(), ServerError> {
        authorized_to_result(matches!(self.inner, UserKind::Constructor(_)))
    }
    /// Retorna um erro se não for driver
    pub fn is_driver(&self) -> Result<(), ServerError> {
        authorized_to_result(matches!(self.inner, UserKind::Driver(_)))
    }
}

/// Essa função transforma uma  boolean em um Ok vazio ou um Err de permissão
/// Assim tenho mais ergonomia pra verificar essas coisas
fn authorized_to_result(authorized: bool) -> Result<(), ServerError> {
    if authorized {
        Ok(())
    } else {
        Err(ServerError::builder()
            .code(Status::Forbidden)
            .message("Você não tem permissão para acessar esse conteúdo.")
            .build())
    }
}

/// Converter da schema da base para o nosso tipo aqui
/// Basicamente vamos pegar os campos da query de user, e fazer a outra query do tipo interno deles
impl TryFrom<Row> for User {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<User, ServerError> {
        // Enum usado na base (sem dados dentro)
        #[derive(FromSql)]
        enum UserType {
            Administrador,
            Escuderia,
            Piloto,
        }

        // Pegar campos da row
        let id = row.try_get("userid")?;
        let login = row.try_get("login")?;
        let kind = row.try_get("tipo")?;

        // Pegar os tipos especificos internos
        // TODO
        let original_id: i32 = row.try_get("idoriginal")?;
        let inner = match kind {
            UserType::Administrador => UserKind::Admin(Admin),
            UserType::Escuderia => UserKind::Constructor(Constructor),
            UserType::Piloto => UserKind::Driver(Driver),
        };

        Ok(User { id, login, inner })
    }
}

impl From<User> for Cookie<'_> {
    fn from(user: User) -> Self {
        Cookie::build("user", user.id.to_string())
            .permanent()
            .finish()
    }
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
