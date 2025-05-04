/*

Authentication Contains:
	-> Login (using username and password)
	-> sign-up (using username, mail_id and password)
	-> forget-password (enter mail_id and using otp, choose a new-password)

*/
use axum::extract::State;
use axum::Form;
use crate::AppState;
use crate::models::authentication::{Login, Mail, SignUp};

pub async fn login(State(state): State<AppState>, Form(login): Form<Login>) {}

pub async fn sign_up(State(state): State<AppState>, Form(sign_up): Form<SignUp>) {}

pub async fn forget_password(State(state): State<AppState>, Form(mail_id): Form<Mail>) {}