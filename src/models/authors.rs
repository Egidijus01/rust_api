use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, FromRow, Clone, ToSchema)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub photo: Option<Vec<u8>>,
    pub created_at: String,
    pub updated_at: String,
}
