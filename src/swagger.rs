use std::sync::Arc;

use crate::models::posts::Post;
use crate::models::authors::Author;
use utoipa::{openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi};
use utoipa_swagger_ui::Config;
use crate::models::response::*;
const DB_URL: &str = "sqlite://sqlite.db";
use crate::handlers::author_handler::{__path_get_all_authors, __path_get_author, __path_delete_author, __path_update_author, __path_post_author};
use crate::handlers::posts_handler::{__path_create_post, __path_get_all_posts, __path_delete_post, __path_download_files_by_id, __path_get_post, __path_get_posts_by_auth, __path_update_post};
use crate::handlers::user_handlers::{__path_register_user_handler, __path_login_user_handler};
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Rejection, Reply,
};
#[derive(OpenApi)]
        #[openapi(
            paths(get_all_authors, post_author, get_author, delete_author, update_author,
             get_post, update_post, get_posts_by_auth, create_post, delete_post, get_all_posts, download_files_by_id,
             register_user_handler, login_user_handler
            ),
            components(
                schemas(
                    Author,
                    Post,
                    CreateAuthorRequest,
                    CreatePostRequest,
                    UpdateAuthorRequest,
                    UpdatePostRequest,
                    AuthorResponse,
                    PostResponse,
                    SingeAuthorResponse,
                    SingePostResponse,
                    UserRequest,
                    LoginResponse,
                    FileResponse
                )
            )
        )]
        pub struct ApiDoc;
    
        struct SecurityAddon;
        impl Modify for SecurityAddon {
            fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
                let components = openapi.components.as_mut().unwrap();
    
                components.add_security_scheme(
                    "bearer_auth",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                );
                components.add_security_scheme(
                    "basic_auth",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Basic)
                            .build(),
                    ),
                );
            }
        }

        pub async fn serve_swagger(
            full_path: FullPath,
            tail: Tail,
            config: Arc<Config<'static>>,
        ) -> Result<Box<dyn Reply + 'static>, Rejection> {
            if full_path.as_str() == "/swagger-ui" {
                return Ok(Box::new(warp::redirect::found(Uri::from_static(
                    "/swagger-ui/",
                ))));
            }
        
            let path = tail.as_str();
            match utoipa_swagger_ui::serve(path, config) {
                Ok(file) => {
                    if let Some(file) = file {
                        Ok(Box::new(
                            Response::builder()
                                .header("Content-Type", file.content_type)
                                .body(file.bytes),
                        ))
                    } else {
                        Ok(Box::new(StatusCode::NOT_FOUND))
                    }
                }
                Err(error) => Ok(Box::new(
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(error.to_string()),
                )),
            }
        }
        
        