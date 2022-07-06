use crate::{
    common::ServerError,
    database::{Client, Row},
    schema::{User, UserKind},
};
use chrono::NaiveDate;
use rocket::http::Status;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Driver {
    id: i32,
    reference: String,
    number: Option<i32>,
    code: Option<String>,
    forename: String,
    surname: String,
    dob: NaiveDate,
    nationality: String,
    url: Option<String>,
}

impl Driver {
    pub async fn from_user(db: &Client, user: &User) -> Result<Driver, ServerError> {
        if user.kind == UserKind::Driver {
            let id = user.original_id.ok_or_else(|| {
                ServerError::builder().message("Esse usuário não tem um id original.")
            })?;
            db.query_one(
                "SELECT driverid, driverref, number, code, forename, surname, dob, nationality, url
                FROM driver
                WHERE driverid = $1",
                &[&id],
            )
            .await?
            .try_into()
        } else {
            Err(ServerError::builder()
                .code(Status::Forbidden)
                .message("Esse usuário não é um piloto.")
                .build())
        }
    }
    pub fn display_name(&self) -> String {
        format!("{} {}", self.forename, self.surname)
    }
    /// Métricas que o piloto tem acesso
    pub async fn get_metrics(&self, db: &Client) -> Result<DriverMetrics, ServerError> {
        db.query_one("SELECT * FROM driver_metrics($1)", &[&self.id])
            .await?
            .try_into()
    }
    /// Relatório 5
    pub async fn get_report5(&self, db: &Client) -> Result<Vec<Report5>, ServerError> {
        db.query("SELECT * FROM report_5($1)", &[&self.id])
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
    /// Relatório 6
    pub async fn get_report6(&self, db: &Client) -> Result<Vec<Report6>, ServerError> {
        db.query("SELECT * FROM report_6($1)", &[&self.id])
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}

/// Converter da schema da base para o nosso tipo aqui
impl TryFrom<Row> for Driver {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Driver, ServerError> {
        Ok(Driver {
            id: row.try_get("driverid")?,
            reference: row.try_get("driverref")?,
            number: row.try_get("number")?,
            code: row.try_get("code")?,
            forename: row.try_get("forename")?,
            surname: row.try_get("surname")?,
            dob: row.try_get("dob")?,
            nationality: row.try_get("nationality")?,
            url: row.try_get("url")?,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct DriverMetrics {
    pub wins: i64,
    pub first_year: Option<i32>,
    pub last_year: Option<i32>,
}

impl TryFrom<Row> for DriverMetrics {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<DriverMetrics, ServerError> {
        Ok(DriverMetrics {
            wins: row.try_get("wins")?,
            first_year: row.try_get("first_year")?,
            last_year: row.try_get("last_year")?,
        })
    }
}

#[derive(Serialize)]
pub struct Report5 {
    pub year: Option<i32>,
    pub race: Option<String>,
    pub wins: i64,
}

impl TryFrom<Row> for Report5 {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Report5, ServerError> {
        Ok(Report5 {
            year: row.try_get("year")?,
            race: row.try_get("race")?,
            wins: row.try_get("wins")?,
        })
    }
}

#[derive(Serialize)]
pub struct Report6 {
    pub status: String,
    pub count: i64,
}

impl TryFrom<Row> for Report6 {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Report6, ServerError> {
        Ok(Report6 {
            status: row.try_get("status")?,
            count: row.try_get("count")?,
        })
    }
}
