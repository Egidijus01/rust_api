use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[allow(non_snake_case)]


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,

}
