use crate::{
    common::ServerError,
    database::{Client, Row},
    schema::{User, UserKind},
};
use rocket::http::Status;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Constructor {
    id: i32,
    reference: String,
    name: String,
    nationality: String,
    url: Option<String>,
}

impl Constructor {
    pub fn display_name(&self) -> String {
        self.name.clone()
    }
    pub async fn from_user(db: &Client, user: &User) -> Result<Constructor, ServerError> {
        if user.kind == UserKind::Constructor {
            let id = user.original_id.ok_or_else(|| {
                ServerError::builder().message("Esse usuário não tem um id original.")
            })?;
            db.query_one(
                "SELECT constructorid, constructorref, name, nationality, url
                FROM constructors
                WHERE constructorid = $1",
                &[&id],
            )
            .await?
            .try_into()
        } else {
            Err(ServerError::builder()
                .code(Status::Forbidden)
                .message("Esse usuário não é uma escuderia.")
                .build())
        }
    }
    /// Métricas que a escuderia tem acesso
    pub async fn get_metrics(&self, db: &Client) -> Result<ConstructorMetrics, ServerError> {
        db.query_one("SELECT * FROM constructor_metrics($1)", &[&self.id])
            .await?
            .try_into()
    }
    /// Relatório 3
    pub async fn get_report3(&self, db: &Client) -> Result<Vec<Report3>, ServerError> {
        db.query("SELECT * FROM report_3($1)", &[&self.id])
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
    /// Relatório 4
    pub async fn get_report4(&self, db: &Client) -> Result<Vec<Report4>, ServerError> {
        db.query("SELECT * FROM report_4($1)", &[&self.id])
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}

/// Converter da schema da base para o nosso tipo aqui
impl TryFrom<Row> for Constructor {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Constructor, ServerError> {
        Ok(Constructor {
            id: row.try_get("constructorid")?,
            reference: row.try_get("constructorref")?,
            name: row.try_get("name")?,
            nationality: row.try_get("nationality")?,
            url: row.try_get("url")?,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct ConstructorMetrics {
    pub wins: i64,
    pub drivers: i64,
    pub first_year: Option<i32>,
    pub last_year: Option<i32>,
}

impl TryFrom<Row> for ConstructorMetrics {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<ConstructorMetrics, ServerError> {
        Ok(ConstructorMetrics {
            wins: row.try_get("wins")?,
            drivers: row.try_get("drivers")?,
            first_year: row.try_get("first_year")?,
            last_year: row.try_get("last_year")?,
        })
    }
}

#[derive(Serialize)]
pub struct Report3 {
    pub name: String,
    pub wins: i64,
}

impl TryFrom<Row> for Report3 {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Report3, ServerError> {
        Ok(Report3 {
            name: row.try_get("name")?,
            wins: row.try_get("wins")?,
        })
    }
}

#[derive(Serialize)]
pub struct Report4 {
    pub status: String,
    pub count: i64,
}

impl TryFrom<Row> for Report4 {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Report4, ServerError> {
        Ok(Report4 {
            status: row.try_get("status")?,
            count: row.try_get("count")?,
        })
    }
}
