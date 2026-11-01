use super::*;
use crate::*;
use color_eyre::eyre::{WrapErr, bail, eyre};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::net::IpAddr;

#[derive(Serialize)]
struct Request<'a, P: Serialize> {
    cmdid: &'a str,
    #[serde(rename = "sessionId", skip_serializing_if = "str::is_empty")]
    session_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<P>,
}

#[derive(Deserialize)]
struct Reply<T> {
    reply: BoxStr,
    params: Option<T>,
    session: Option<BoxStr>,
}

pub struct Router {
    client: reqwest::Client,
    endpoint: BoxStr,
    session: BoxStr,
    format: Format,
}

impl Router {
    pub async fn login(
        router: &IpAddr,
        username: &str,
        password: &str,
        format: &Format,
    ) -> EyreResult<Self> {
        let mut this = Self {
            client: reqwest::Client::builder().user_agent("Kimem CLI").build()?,
            endpoint: format!("http://{router}/himiapi/json").into_boxed_str(),
            session: "".into(),
            format: format.to_owned(),
        };

        #[derive(Serialize)]
        struct Login<'a> {
            cmdid: &'a str,
            username: &'a str,
            password: &'a str,
        }

        let body = Login {
            cmdid: "login",
            username,
            password,
        };
        let reply: Reply<serde_json::Value> = this.send("login", &body).await?;

        match (reply.reply.as_ref(), reply.session) {
            ("ok", Some(session)) => {
                this.session = session;
                Ok(this)
            }
            ("ok", None) => bail!("login failed: router did not return a session"),
            (other, _) => bail!("login failed: wrong username or password (reply {other:?})"),
        }
    }

    async fn post_once(&self, body: Vec<u8>) -> EyreResult<String> {
        let response = self
            .client
            .post(self.endpoint.as_ref())
            .header(CONTENT_TYPE, "application/json;charset=UTF-8")
            .header(AUTHORIZATION, self.session.as_ref())
            .body(body)
            .send()
            .await?;

        Ok(response.text().await?)
    }

    async fn send<B: Serialize, T: DeserializeOwned>(
        &self,
        cmdid: &str,
        body: &B,
    ) -> EyreResult<Reply<T>> {
        let body = serde_json::to_vec(body)?;
        let text = with_retry(|| self.post_once(body.clone())).await?;
        decode(cmdid, &text)
    }

    pub async fn call<P: Serialize, T: DeserializeOwned>(
        &self,
        cmdid: &str,
        params: Option<P>,
    ) -> EyreResult<Option<T>> {
        let reply: Reply<T> = self.send(cmdid, &self.request(cmdid, params)).await?;
        check(cmdid, reply)
    }

    fn request<'a, P: Serialize>(&'a self, cmdid: &'a str, params: Option<P>) -> Request<'a, P> {
        Request {
            cmdid,
            session_id: &self.session,
            params,
        }
    }

    pub async fn get<T: HimiGet>(&self) -> EyreResult<T> {
        self.call::<(), T>(T::CMDID, None)
            .await?
            .ok_or_else(|| eyre!("router returned no data for {:?}", T::CMDID))
    }

    pub async fn show<T: HimiGet + Show>(&self) -> EyreResult<()> {
        self.get::<T>().await?.show(&self.format)
    }

    pub async fn show_network(&self) -> EyreResult<()> {
        NetworkReport {
            mode: self.get::<NetworkModeInfo>().await?,
            sim: self.get::<SimSlot>().await?,
            usb: self.get::<UsbTether>().await?,
            detect: self.get::<ConnectionDetect>().await?,
        }
        .show(&self.format)
    }

    pub async fn logout(&self) -> EyreResult<()> {
        self.call::<(), serde_json::Value>("logout", None).await?;
        Ok(())
    }

    /// Sent exactly once: a retry after a lost reply would reboot twice.
    pub async fn reboot(&self) -> EyreResult<()> {
        const CMDID: &str = "rebootcmd";

        let body = serde_json::to_vec(&self.request(CMDID, Some("reboot")))?;
        let text = self.post_once(body).await?;
        check::<serde_json::Value>(CMDID, decode(CMDID, &text)?)?;

        println!("Device rebooting.");
        Ok(())
    }
}

fn decode<T: DeserializeOwned>(cmdid: &str, text: &str) -> EyreResult<Reply<T>> {
    if text.trim().is_empty() {
        bail!("router does not support the {cmdid:?} command");
    }

    serde_json::from_str(text)
        .wrap_err_with(|| format!("could not decode router response to {cmdid:?}: {text}"))
}

fn check<T>(cmdid: &str, reply: Reply<T>) -> EyreResult<Option<T>> {
    match reply.reply.as_ref() {
        "ok" => Ok(reply.params),
        "SessionOut" => bail!("router session expired (another login may have replaced it)"),
        other => bail!("router rejected {cmdid:?}: {other}"),
    }
}
