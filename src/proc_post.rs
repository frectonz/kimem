use crate::common::*;
use serde::{Deserialize, Serialize};

pub trait ProcPost: serde::de::DeserializeOwned {
    const GOFORM_ID: &str;
    type Params: serde::ser::Serialize + Default;

    fn print_table(&self) -> EyreResult<()>;
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
    const GOFORM_ID: &str = "LOGIN";
    type Params = LoginParams;

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Result", "Power", "Unique Login Credentials"])
            .add_row([&self.result, &self.power, &self.unique_login_credentials]);

        println!("{table}");
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Logout {
    pub result: BoxStr,
}

impl ProcPost for Logout {
    const GOFORM_ID: &str = "LOGOUT";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Logout Result"]).add_row([&self.result]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct RebootDevice;

impl ProcPost for RebootDevice {
    const GOFORM_ID: &str = "REBOOT_DEVICE";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        println!("Device Rebooted.");
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteSms {
    pub result: BoxStr,
}

#[derive(Debug, Serialize, Default)]
pub struct DeleteSmsParams {
    pub msg_id: BoxStr,
}

impl ProcPost for DeleteSms {
    const GOFORM_ID: &str = "DELETE_SMS";
    type Params = DeleteSmsParams;

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Delete Result"]).add_row([&self.result]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct SendSms {
    pub result: BoxStr,
}

#[derive(Debug, Serialize, Default)]
pub struct SendSmsParams {
    #[serde(rename = "notCallback")]
    not_callback: BoxStr,
    encode_type: BoxStr,
    #[serde(rename = "ID")]
    id: BoxStr,

    #[serde(rename = "Number")]
    number: BoxStr,
    #[serde(rename = "MessageBody")]
    message_body: BoxStr,
    sms_time: BoxStr,
}

impl SendSmsParams {
    pub fn new(number: &str, message: &str) -> Self {
        Self {
            not_callback: "true".into(),
            encode_type: "UNICODE".into(),
            id: "-1".into(),

            number: number.into(),
            sms_time: Datetime::now().router_time(),
            message_body: ucs2_encode(message),
        }
    }
}

impl ProcPost for SendSms {
    const GOFORM_ID: &str = "SEND_SMS";
    type Params = SendSmsParams;

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Send Result"]).add_row([&self.result]);
        println!("{table}");

        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub struct MarkSms {
    pub result: BoxStr,
}

#[derive(Debug, Serialize, Default)]
pub struct MarkSmsParams {
    pub msg_id: BoxStr,
    pub tag: BoxStr,
}

impl ProcPost for MarkSms {
    const GOFORM_ID: &str = "SET_MSG_READ";
    type Params = MarkSmsParams;

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Mark Result"]).add_row([&self.result]);
        println!("{table}");

        Ok(())
    }
}
