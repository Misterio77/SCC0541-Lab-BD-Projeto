use crate::{
    common::ServerError,
    database::{Client, Row},
    schema::{SpecializedUser, User, UserKind},
};
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Admin(());

#[async_trait::async_trait]
impl SpecializedUser for Admin {
    type Output = Admin;
    /// Criar o admin
    async fn from_user(_db: &Client, user: &User) -> Result<Admin, ServerError> {
        // Pegamos a database mas não usamos, só pra manter a interface consistente
        if user.kind == UserKind::Admin {
            Ok(Admin(()))
        } else {
            Err(ServerError::forbidden())
        }
    }

    type Metrics = AdminMetrics;
    /// Operações ou métricas que o admin tem acesso
    async fn get_metrics(&self, db: &Client) -> Result<AdminMetrics, ServerError> {
        db.query_one(
            "SELECT
                (SELECT count(*) FROM driver) AS drivers,
                (SELECT count(*) FROM constructors) AS constructors,
                (SELECT count(*) FROM races) AS races,
                (SELECT count(*) FROM seasons) AS seasons",
            &[],
        )
        .await?
        .try_into()
    }

    fn display_name(&self) -> String {
        "Admin".into()
    }
}

#[derive(Serialize, Debug)]
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
