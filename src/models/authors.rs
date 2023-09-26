use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]

pub struct Author {
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub created_at: String,
    pub updated_at: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateAuthorSchema {
    pub name: String,
    pub surname: String,
}
