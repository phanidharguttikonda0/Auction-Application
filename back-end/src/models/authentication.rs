use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use lazy_static::lazy_static;
use regex::Regex;
use chrono::{NaiveDate, NaiveTime};

lazy_static! {
    static ref USERNAME_RE: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}
#[derive(Deserialize, Debug, Validate)]
pub struct SignUp {
    #[validate(length(min = 3, max = 18, message = "Username must be between 3 and 15 characters"))]
    #[validate(regex(path = *USERNAME_RE, message = "Username can only contain alphanumeric characters and underscores"))]
    pub username: String,
    #[validate(email)]
    pub mail_id: String,
    #[validate(length(min = 8, message = "Password should be at least 8 characters long"))]
    pub password: String,
    pub dob: NaiveDate
}

#[derive(Deserialize, Debug, Validate)]
pub struct Login {
    #[validate(length(min = 3, max = 18, message = "Username must be between 3 and 15 characters"))]
    #[validate(regex(path = *USERNAME_RE, message = "Username can only contain alphanumeric characters and underscores"))]
    pub username: String,
    #[validate(length(min = 8, message = "Password should be at least 8 characters long"))]
    pub password: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct Mail {
    #[validate(email)]
    pub mail_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub authorization: String,
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Claims {
    pub username: String,
    pub user_id: i32,
    pub exp: i64,
}
