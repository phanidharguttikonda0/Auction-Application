/*

Authentication Contains:
	-> Login (using username and password)
	-> sign-up (using username, mail_id and password)
	-> forget-password (enter mail_id and using otp, choose a new-password)

*/
use axum::extract::State;
use axum::{Form, Json};
use sqlx::Error;
use validator::Validate;
use crate::AppState;
use crate::middlewares::authentication::{get_authorization_header, hash_password};
use crate::models::authentication::{Login, Mail, SignUp, Token};


pub async fn login(State(state): State<AppState>, Form(login): Form<Login>) -> Json<Result<Token, String>> {

    if let Err(validation_error) = login.validate() {
        tracing::error!("{}", validation_error) ;
        return Json::from(Err(format!("{}", validation_error)))
    }

    // password should be in encrypted format
    let value = sqlx::query_scalar::<_,i32>("select id from users where username=$1 and password=$2")
        .bind(&login.username).bind(&hash_password(login.password)).fetch_optional(&state.sql_database).await.unwrap() ;

    match value {
        Some(value) => {
            tracing::info!("Successfully logged in") ;
            Json::from(Ok(Token {
                authorization: get_authorization_header(login.username, value).await
            }))
        },
        None => {
            tracing::info!("invalid credentials") ;
            Json::from(Err(String::from("Invalid Credentials")))
        }
    }

}

pub async fn sign_up(State(state): State<AppState>, Form(sign_up): Form<SignUp>) -> Json<Result<Token,String>> {

    if let Err(validation_error) = sign_up.validate() {
        tracing::error!("{}", validation_error) ;
        return Json::from(Err(format!("{}", validation_error)))
    }
    tracing::info!("length of the hashed password was {}", hash_password(sign_up.password.clone()).len()) ;
    let value = sqlx::query_scalar::<_,i32>("insert into users(username,password,mail_id,DOB) values($1,$2,$3,$4) returning id", )
        .bind(&sign_up.username).bind(&hash_password(sign_up.password)).bind(&sign_up.mail_id).bind(sign_up.dob)
        .fetch_one(&state.sql_database).await ;

    match value {
        Ok(val) => {
            tracing::info!("successfully signed up");
            Json::from(Ok(
                Token{
                    authorization: get_authorization_header(sign_up.username, val).await
                }
            ))
        },
        Err(err) => {
            tracing::error!("Error in Sign-Up {}", err) ;
            Json::from(Err(String::from("Details Already Exists")))
        }
    }

}

pub async fn forget_password(State(state): State<AppState>, Form(mail_id): Form<Mail>)  {

}