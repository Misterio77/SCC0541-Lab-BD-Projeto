/// Muito desse código veio de outros projetos meus
/// É basicamente bastante boilerplate pra integrar diferentes tipos de erro
/// e converter entre eles.
///
/// Bem overengineered, mas pq não se eu já sabia fazer?
pub use rocket::{
    http::{uri::Reference, MediaType, Status},
    outcome::{IntoOutcome, Outcome},
    response::{Responder, Response},
};

use std::error::Error as StdError;
use std::fmt;

use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

/// Tipo principal de erros
///
/// Esse é o erro unificado para tudo, ele tem um builder pra facilitar construção
/// E tem vários métodos pra converter pra outros tipos de uso conveniente
#[derive(Debug)]
pub struct ServerError {
    code: Status,
    source: Option<Box<dyn StdError + Sync + Send>>,
    message: Option<String>,
}

impl ServerError {
    /// Iniciar um builder a partir do zero
    pub fn builder() -> ServerErrorBuilder {
        ServerError::default().edit()
    }
    /// Transformar em builder, para editar de forma ergonomica
    pub fn edit(self) -> ServerErrorBuilder {
        ServerErrorBuilder { inner: self }
    }
    /// Combina builder e from (conversão). Então converte algo nele e daí abre para edição
    pub fn builder_from<T: Into<ServerError>>(source: T) -> ServerErrorBuilder {
        source.into().edit()
    }
    /// Transforma num Flash<Redirect>
    /// A maioria das coisas que podem dar errado vão ser exibidas como um redirecionamento +
    /// mensagem de erro amigável (vulgo flash)
    pub fn flash_redirect<U: TryInto<Reference<'static>>>(&self, uri: U) -> Flash<Redirect> {
        let message = self.message.as_deref().unwrap_or("Erro desconhecido.");
        Flash::error(Redirect::to(uri), message)
    }

    // Alguns erros comuns que me pego construindo toda hora
    pub fn forbidden() -> ServerError {
        ServerError::builder()
            .message("Você não tem permissão para acessar esse recurso")
            .code(Status::Forbidden)
            .build()
    }
}

/// Construtor padrão
impl Default for ServerError {
    fn default() -> Self {
        ServerError {
            code: Status::InternalServerError,
            source: None,
            message: None,
        }
    }
}

/// Implementação pra pretty print
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code.reason_lossy())?;
        if let Some(message) = &self.message {
            write!(f, " ({})", message)?;
        };
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        };
        Ok(())
    }
}

/// Implementar a interface de erro genérica
impl StdError for ServerError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|s| &**s as _)
    }
}

/// Builder pode virar ele usando from também
impl From<ServerErrorBuilder> for ServerError {
    fn from(e: ServerErrorBuilder) -> Self {
        e.build()
    }
}

/// Converte um Option<Into<ServerError>>. Isso é, um tipo que pode (se não for None) ser
/// convertido no ServerErro. BEM específico, mas novamente, ergonomia pra alguns casos.
impl<T: Into<ServerError>> From<Option<T>> for ServerError {
    fn from(e: Option<T>) -> Self {
        match e {
            Some(error) => ServerError::builder_from(error),
            None => ServerError::builder(),
        }
        .build()
    }
}

/// Permite converter uma tupla (status http, algo que pode virar o ServerError)
/// num ServerError. Isso é util pra converter entre ele e o Outcome do Rocket
impl<T: Into<ServerError>> From<(Status, T)> for ServerError {
    fn from(e: (Status, T)) -> Self {
        ServerError::builder_from(e.1).code(e.0).build()
    }
}

/// Inverso do acima, útil pra transformar num Outcome
impl From<ServerError> for (Status, ServerError) {
    fn from(e: ServerError) -> Self {
        (e.code, e)
    }
}

/// Converter erros de inicialização de pool de conexão com a base
impl<
        T: Send + Sync + fmt::Display + fmt::Debug + 'static,
        U: Send + Sync + fmt::Display + fmt::Debug + 'static,
    > From<rocket_db_pools::Error<T, U>> for ServerError
{
    fn from(e: rocket_db_pools::Error<T, U>) -> Self {
        ServerError::builder()
            .code(Status::ServiceUnavailable)
            .source(Box::new(e))
            .message("Não foi possível iniciar pool de conexões com a base")
            .build()
    }
}

/// Converter erros do Rocket
impl From<rocket::error::Error> for ServerError {
    fn from(e: rocket::error::Error) -> Self {
        ServerError::builder()
            .code(Status::ServiceUnavailable)
            .source(Box::new(e))
            .message("Não foi possível iniciar o servidor")
            .build()
    }
}
/// Para erros propagados da base, vamos fazer um tratamento legal pra ter mensagens úteis
impl From<rocket_db_pools::deadpool_postgres::tokio_postgres::Error> for ServerError {
    fn from(e: rocket_db_pools::deadpool_postgres::tokio_postgres::Error) -> Self {
        let (message, code) = match e.as_db_error() {
            Some(db_e) => (db_e.to_string(), Status::InternalServerError),
            None => match e.to_string().as_str() {
                "query returned an unexpected number of rows" => {
                    ("Recurso não encontrado".into(), Status::NotFound)
                }
                e => (e.into(), Status::InternalServerError),
            },
        };

        ServerError::builder()
            .message(&message)
            .code(code)
            .source(Box::new(e))
            .build()
    }
}

/// Permite retornar o erro diretamente de uma rota, ele vai pro template de erro.
impl<'r> Responder<'r, 'static> for ServerError {
    fn respond_to(
        self,
        req: &'r rocket::request::Request<'_>,
    ) -> rocket::response::Result<'static> {
        let code = self.code;

        let response = Template::render("error", self).respond_to(req)?;

        rocket::response::Response::build()
            .status(code)
            .join(response)
            .ok()
    }
}

/// Permite serializar em qualquer formato
impl Serialize for ServerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ServerError", 2)?;
        state.serialize_field("code", &format!("{}", &self.code))?;
        state.serialize_field("description", &self.message)?;
        state.serialize_field(
            "reason",
            &self
                .source
                .as_ref()
                .map(|s| format!("{:?}", s).replace("\"", "'").replace("\\'", "'")),
        )?;
        state.end()
    }
}

/// Builder
/// Isso segue a pattern builder pra construir
/// o tipo ServerError de forma ergonômica
pub struct ServerErrorBuilder {
    inner: ServerError,
}

impl ServerErrorBuilder {
    /// Finalizar a build
    pub fn build(self) -> ServerError {
        self.inner
    }

    /// Adicionar código de erro HTTP
    pub fn code(self, code: Status) -> ServerErrorBuilder {
        ServerErrorBuilder {
            inner: ServerError { code, ..self.inner },
        }
    }
    /// Adicionar fonte do erro
    pub fn source(self, source: Box<dyn StdError + Sync + Send>) -> ServerErrorBuilder {
        ServerErrorBuilder {
            inner: ServerError {
                source: Some(source),
                ..self.inner
            },
        }
    }
    /// Adicionar mensagem amigável
    pub fn message(self, message: &str) -> ServerErrorBuilder {
        ServerErrorBuilder {
            inner: ServerError {
                message: Some(message.into()),
                ..self.inner
            },
        }
    }
}
