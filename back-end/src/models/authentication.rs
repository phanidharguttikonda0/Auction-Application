use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SignUp {
    pub username: String,
    pub mail_id: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Mail {
    pub mail_id: String,
}

