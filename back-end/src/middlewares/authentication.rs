use axum::middleware::Next;
use axum::{extract::Request, http::StatusCode, response::Response} ;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sha2::Digest;
use crate::models::authentication::Claims;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn authorization_check(mut req: Request, next: Next ) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get("Authorization");
    match auth_header {
        Some(auth_header) => {
            tracing::info!("{}", auth_header.to_str().unwrap());
            let data = authorization_decode(auth_header.to_str().unwrap().to_string()).unwrap();
            req.extensions_mut().insert(data) ; // where in request extensions we will add the data from the middlewares to it
            // in handlers we can get the data we added in Extensions by using Extensions extractor as follows
            // handler(Extensions(data) : Extensions<Claims>) so it returns the Claims type data that we inserted to the extensions
            Ok(next.run(req).await)
        },
        None => {
            tracing::error!("No authorization header found");
            Err(StatusCode::UNAUTHORIZED)
        }
    }

}

pub fn authorization_decode(token: String) -> Option<Claims> {
    if !token.starts_with("Bearer ") {
        tracing::error!("Invalid authorization header with no Bearer");
        return None
    }
    let token = token.replace("Bearer ", "");
    tracing::info!("Authorization header found: {}", token);
    let data = decode::<Claims>(&token, &DecodingKey::from_secret(b"phani"), &Validation::default()).unwrap() ;
    Some(data.claims)
}

pub async fn get_authorization_header(username: String, user_id: i32) -> String {

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap()
        .as_secs() + 3600000; // valid for 1000 hours
    let header = encode(&Header::default(),&Claims{ username, user_id, exp:expiration as i64 }, &EncodingKey::from_secret(b"phani"));
    match header {
        Ok(token) => {
            format!("Bearer {}", token)
        },
        Err(_) => { String::from("") }
    }
}


pub fn hash_password(password: String) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}