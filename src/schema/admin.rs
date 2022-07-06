use crate::{
    common::ServerError,
    database::{Client, Row},
    schema::{User, UserKind},
};
use rocket::http::Status;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq)]
pub struct Admin(());

impl Admin {
    pub fn display_name(&self) -> String {
        "Admin".into()
    }
    pub async fn from_user(_db: &Client, u: &User) -> Result<Admin, ServerError> {
        if u.kind == UserKind::Admin {
            Ok(Admin(()))
        } else {
            Err(ServerError::builder()
                .code(Status::Forbidden)
                .message("Esse usuário não é administrador.")
                .build())
        }
    }
    /// Métricas que o admin tem acesso
    pub async fn get_metrics(&self, db: &Client) -> Result<AdminMetrics, ServerError> {
        db.query_one("SELECT * FROM admin_metrics()", &[])
            .await?
            .try_into()
    }
    /// Relatório 1
    pub async fn get_report1(&self, db: &Client) -> Result<Vec<Report1>, ServerError> {
        db.query("SELECT * FROM report_1()", &[])
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
    /// Relatório 2
    pub async fn get_report2(&self, db: &Client, city: &str) -> Result<Vec<Report2>, ServerError> {
        db.query("SELECT * FROM report_2($1)", &[&city])
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}

#[derive(Serialize)]
pub struct AdminMetrics {
    pub drivers: i64,
    pub constructors: i64,
    pub races: i64,
    pub seasons: i64,
}

impl TryFrom<Row> for AdminMetrics {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<AdminMetrics, ServerError> {
        Ok(AdminMetrics {
            drivers: row.try_get("drivers")?,
            constructors: row.try_get("constructors")?,
            races: row.try_get("races")?,
            seasons: row.try_get("seasons")?,
        })
    }
}

#[derive(Serialize)]
pub struct Report1 {
    pub status: String,
    pub count: i64,
}

impl TryFrom<Row> for Report1 {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Report1, ServerError> {
        Ok(Report1 {
            status: row.try_get("status")?,
            count: row.try_get("count")?,
        })
    }
}

#[derive(Serialize)]
pub struct Report2 {
    pub city_name: String,
    pub airport_name: String,
    pub airport_iata: String,
    pub airport_city: String,
}

impl TryFrom<Row> for Report2 {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Report2, ServerError> {
        Ok(Report2 {
            city_name: row.try_get("city_name")?,
            airport_name: row.try_get("airport_name")?,
            airport_iata: row.try_get("airport_iata")?,
            airport_city: row.try_get("airport_city")?,
        })
    }
}
