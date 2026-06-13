use crate::common::*;
use serde::{Deserialize, Serialize};

pub trait ProcPost {
    const GOFROM_ID: &str;
    type Params: serde::ser::Serialize + Default;
    type Response: serde::de::DeserializeOwned;
}

#[derive(Debug, Serialize, Default)]
pub struct LoginParams {
    pub username: BoxStr,
    pub password: BoxStr,
    pub unique_login_credentials: BoxStr,
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub result: BoxStr,
    pub power: BoxStr,
    pub unique_login_credentials: BoxStr,
}

impl ProcPost for Login {
    const GOFROM_ID: &str = "LOGIN";
    type Params = LoginParams;
    type Response = Login;
}

#[derive(Debug, Deserialize)]
pub struct Logout {
    pub result: BoxStr,
}

impl ProcPost for Logout {
    const GOFROM_ID: &str = "LOGOUT";
    type Params = ();
    type Response = Logout;
}
