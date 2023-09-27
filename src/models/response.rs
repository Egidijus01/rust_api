use crate::models::{authors::Author, posts::Post};

use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct SingeAuthorResponse {
    pub status: String,
    pub data: Author,
}

#[derive(Serialize, Debug)]
pub struct SingePostResponse {
    pub status: String,
    pub data: Post,
}





#[derive(Serialize,Debug)]
pub struct AuthorResponse {
    pub status: String,
    pub results: usize,
    pub authors: Vec<Author>
}


#[derive(Serialize,Debug)]
pub struct PostResponse {
    pub status: String,
    pub results: usize,
    pub posts: Vec<Post>
}

#[derive(Deserialize, Debug)]
pub struct CreateAuthorRequest {
    pub name: String,
    pub surname: String,
    pub photo: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub uploaded_file: Option<Vec<u8>>
}

#[derive(Deserialize, Debug)]
pub struct UserRequest {
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Debug)]
pub struct UpdateAuthorRequest{
    pub name: String,
    pub surname: String,
    pub photo: Option<Vec<u8>,>

}
#[derive(Deserialize, Debug)]

pub struct UpdatePostRequest{
    pub title: String,
    pub content: String,



}


#[derive(Serialize, Debug)]
pub struct StatusResponse {
    pub status: String,
}


#[derive(Serialize)]
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
