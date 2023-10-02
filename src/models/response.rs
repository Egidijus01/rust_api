use crate::models::{authors::Author, posts::Post};

use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct SingeAuthorResponse {
    pub status: String,
    pub data: Author,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct SingePostResponse {
    pub status: String,
    pub data: Post,
}





#[derive(Serialize,Debug, ToSchema)]
pub struct AuthorResponse {
    pub status: String,
    pub results: usize,
    pub authors: Vec<Author>
}


#[derive(Serialize,Debug, ToSchema)]
pub struct PostResponse {
    pub status: String,
    pub results: usize,
    pub posts: Vec<Post>
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateAuthorRequest {
    #[schema(example = "John", required = true)]
    pub name: String,
    #[schema(example = "Doe", required = true)]
    pub surname: String,
    #[schema(example = "Base64 photo (Optional)", required = false)]
    pub photo: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreatePostRequest {
    #[schema(example = "Hello", required = true)]
    pub title: String,
    #[schema(example = "World", required = true)]
    pub content: String,
    #[schema(example = "1", required = true)]
    pub author_id: i64,
    #[schema(example = "files in ['Base64', 'Base64'] format (Optional)", required = false)]
    pub uploaded_files: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UserRequest {
    #[schema(example = "John", required = true)]
    pub username: String,
    #[schema(example = "Doe", required = true)]
    pub password: String
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateAuthorRequest{
    #[schema(example = "John", required = true)]
    pub name: String,
    #[schema(example = "Doe", required = true)]
    pub surname: String,
    #[schema(example = "Base64 photo (Optional)", required = false)]
    pub photo: Option<String>,


}
#[derive(Deserialize, Debug, ToSchema)]

pub struct UpdatePostRequest{
    #[schema(example = "Hello", required = true)]
    pub title: String,
    #[schema(example = "World", required = true)]
    pub content: String,
    #[schema(example = "files in ['Base64', 'Base64'] format (Optional)", required = false)]
    pub uploaded_files: Option<Vec<String>>,

}


#[derive(Serialize, Debug)]
pub struct StatusResponse {
    pub status: String,
}


#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct PageQueryParam {
    pub page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQueryParam{
    pub search: Option<String>
}

#[derive (Debug, Serialize, ToSchema)]
pub struct FileResponse{
    pub status: String,
    pub data: Vec<String>,
}