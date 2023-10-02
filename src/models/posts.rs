use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
#[allow(non_snake_case)]


#[derive(Clone, FromRow, Debug, Serialize, Deserialize, ToSchema)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

