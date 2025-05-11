use axum::middleware::Next;
use axum::{extract::Request, http::StatusCode, response::Response} ;
use sha2::Digest;

pub async fn authorization_check(req: Request,next: Next ) -> Result<Response, StatusCode> {
    Ok(next.run(req).await)
}

pub async fn get_authorization_header(username: String, user_id: i32) -> String {
    String::from("Bearer ")
}

pub async fn validate_details(req: Request, next: Next) -> Result<Response, StatusCode> {
    // need to handle sign-up, login and forgot password details one by one by checking which request was it
    Ok(next.run(req).await)
}

pub fn hash_password(password: String) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}