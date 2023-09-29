const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use chrono::Utc; // Import Utc from the chrono crate
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Filter, Rejection,
    
};
use warp::reject::Reject;

#[derive(Debug)]
pub enum MyError {
    JWTTokenError,
    JWTTokenCreationError,
    NoAuthHeaderError,
    InvalidAuthHeaderError,
    // Add more custom error variants as needed
}
impl Reject for MyError {}
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(username: &String) -> Result<String, MyError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(20))  // 5 MINUTE LIFESPAN
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| MyError::JWTTokenCreationError)
}



fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, MyError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(MyError::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(MyError::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(MyError::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}

async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<String, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| warp::reject::custom(MyError::JWTTokenError))?;

            Ok(decoded.claims.sub)
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| headers)
        .and_then(authorize)
}