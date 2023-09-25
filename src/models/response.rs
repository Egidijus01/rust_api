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
}

#[derive(Deserialize, Debug)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub author_id: i64,
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

}
#[derive(Deserialize, Debug)]

pub struct UpdatePostRequest{
    pub title: String,
    pub content: String,



}


#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
}


#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}