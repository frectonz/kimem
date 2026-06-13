use crate::common::*;
use serde::{Deserialize, Serialize};

pub trait ProcPost {
    const GOFROM_ID: &str;
    type Params: serde::ser::Serialize + Default;
    type Response: serde::de::DeserializeOwned;

    fn print_table(&self);
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

    fn print_table(&self) {
        let mut table = comfy_table::Table::new();

        table
            .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
            .set_header(["Result", "Power", "Unique Login Credentials"])
            .add_row([&self.result, &self.power, &self.unique_login_credentials]);

        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct Logout {
    pub result: BoxStr,
}

impl ProcPost for Logout {
    const GOFROM_ID: &str = "LOGOUT";
    type Params = ();
    type Response = Logout;

    fn print_table(&self) {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
pub struct RebootDevice;

impl ProcPost for RebootDevice {
    const GOFROM_ID: &str = "REBOOT_DEVICE";
    type Params = ();
    type Response = RebootDevice;

    fn print_table(&self) {
        todo!()
    }
}
