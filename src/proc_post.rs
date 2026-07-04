use crate::common::*;
use crate::types::Datetime;
use serde::{Deserialize, Deserializer, Serialize};

/// An action on `/reqproc/proc_post` identified by a `goformId`.
pub trait ProcPost: serde::de::DeserializeOwned {
    const GOFORM_ID: &str;
    type Params: serde::ser::Serialize + Default + Send + Sync;
}

/// Batch message IDs the way the router's web UI does: "1;2;3;"
/// (the trailing separator is required).
pub fn join_msg_ids(ids: impl IntoIterator<Item = usize>) -> BoxStr {
    use std::fmt::Write;

    let mut joined = String::new();
    for id in ids {
        write!(joined, "{id};").expect("writing to a String never fails");
    }

    joined.into_boxed_str()
}

#[derive(Debug, Serialize, Default)]
pub struct LoginParams {
    pub username: BoxStr,
    pub password: BoxStr,
    pub unique_login_credentials: BoxStr,
}

#[derive(Debug, Clone)]
pub enum LoginResult {
    Success,
    InvalidCredentials,
    MalformedRequest,
    Other(BoxStr),
}

impl<'de> Deserialize<'de> for LoginResult {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = BoxStr::deserialize(deserializer)?;
        Ok(match raw.as_ref() {
            "0" => Self::Success,
            "3" => Self::InvalidCredentials,
            "1" => Self::MalformedRequest,
            _ => Self::Other(raw),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub result: LoginResult,
}

impl ProcPost for Login {
    const GOFORM_ID: &str = "LOGIN";
    type Params = LoginParams;
}

#[derive(Debug, Deserialize)]
pub struct Logout {}

impl ProcPost for Logout {
    const GOFORM_ID: &str = "LOGOUT";
    type Params = ();
}

/// The response is never read: the router kills the connection mid-reboot.
#[derive(Debug, Deserialize)]
pub struct RebootDevice {}

impl ProcPost for RebootDevice {
    const GOFORM_ID: &str = "REBOOT_DEVICE";
    type Params = ();
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
}

impl Show for DeleteSms {
    fn show(&self) -> EyreResult<()> {
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
}

impl Show for SendSms {
    fn show(&self) -> EyreResult<()> {
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

impl MarkSmsParams {
    pub fn new(msg_id: BoxStr) -> Self {
        Self {
            msg_id,
            tag: "0".into(),
        }
    }
}

impl ProcPost for MarkSms {
    const GOFORM_ID: &str = "SET_MSG_READ";
    type Params = MarkSmsParams;
}

impl Show for MarkSms {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Mark Result"]).add_row([&self.result]);
        println!("{table}");

        Ok(())
    }
}
