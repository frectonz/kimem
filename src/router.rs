use crate::*;
use reqwest::header::REFERER;
use serde::Serialize;
use std::net::IpAddr;

#[derive(Debug, Serialize)]
#[serde(tag = "isTest", rename = "false")]
struct ParamsBody<T: serde::Serialize> {
    cmd: BoxStr,
    #[serde(flatten)]
    payload: T,
}

#[derive(Debug, Serialize)]
#[serde(tag = "isTest", rename = "false")]
struct FormBody<T: serde::Serialize> {
    #[serde(rename = "goformId")]
    goform_id: BoxStr,
    #[serde(flatten)]
    payload: T,
}

pub struct Router {
    client: reqwest::Client,
    address: BoxStr,
    username: BoxStr,
    password: BoxStr,
}

impl Router {
    pub fn new(router: &IpAddr, username: &str, password: &str) -> EyreResult<Self> {
        Ok(Self {
            client: reqwest::Client::builder().user_agent("Kimem CLI").build()?,
            address: format!("http://{router}").into_boxed_str(),
            username: username.into(),
            password: password.into(),
        })
    }

    async fn create_get<Req: ProcGet>(&self, params: Req::Params) -> EyreResult<reqwest::Response> {
        let address = self.address.as_ref();

        let params = serde_urlencoded::to_string(&ParamsBody {
            cmd: Req::CMD.into(),
            payload: params,
        })?;

        let url = format!("{address}/reqproc/proc_get?{params}");

        with_retry(|| async { self.client.get(&url).send().await.map_err(Into::into) }).await
    }

    pub async fn get_with<Req: ProcGet>(&self, params: Req::Params) -> EyreResult<Req> {
        let body = self.create_get::<Req>(params).await?.json::<Req>().await?;
        Ok(body)
    }

    pub async fn get_text_with<Req: ProcGet>(&self, params: Req::Params) -> EyreResult<BoxStr> {
        let body = self
            .create_get::<Req>(params)
            .await?
            .text()
            .await?
            .into_boxed_str();

        Ok(body)
    }

    pub async fn get<Req: ProcGet>(&self) -> EyreResult<Req> {
        self.get_with::<Req>(Req::Params::default()).await
    }

    pub async fn get_text<Req: ProcGet>(&self) -> EyreResult<BoxStr> {
        self.get_text_with::<Req>(Req::Params::default()).await
    }

    pub async fn create_post<Req: ProcPost>(
        &self,
        params: Req::Params,
    ) -> EyreResult<reqwest::Response> {
        let address = self.address.as_ref();
        let url = format!("{address}/reqproc/proc_post");

        let form = FormBody {
            goform_id: Req::GOFORM_ID.into(),
            payload: params,
        };

        with_retry(|| async {
            self.client
                .post(&url)
                .form(&form)
                .header(REFERER, self.address.as_ref())
                .send()
                .await
                .map_err(Into::into)
        })
        .await
    }

    pub async fn post_with<Req: ProcPost>(&self, params: Req::Params) -> EyreResult<Req> {
        let body = self.create_post::<Req>(params).await?.json::<Req>().await?;
        Ok(body)
    }

    pub async fn post_text_with<Req: ProcPost>(&self, params: Req::Params) -> EyreResult<BoxStr> {
        let body = self
            .create_post::<Req>(params)
            .await?
            .text()
            .await?
            .into_boxed_str();

        Ok(body)
    }

    pub async fn post<Req: ProcPost>(&self) -> EyreResult<Req> {
        self.post_with::<Req>(Req::Params::default()).await
    }

    pub async fn post_text<Req: ProcPost>(&self) -> EyreResult<BoxStr> {
        self.post_text_with::<Req>(Req::Params::default()).await
    }

    pub async fn login(&self) -> EyreResult<Login> {
        let password = self.password.as_ref();
        let random_login = self.get::<GetRandomLogin>().await?;
        let nonce = random_login.random_login;

        let form = LoginParams {
            username: b64_encode(&self.username),
            password: b64_encode(&sha256_encode(&format!("{nonce}{password}"))),
            unique_login_credentials: "1".into(),
        };

        let body = self.post_with::<Login>(form).await?;

        Ok(body)
    }

    pub async fn logout(&self) -> EyreResult<Logout> {
        self.post::<Logout>().await
    }

    pub async fn reboot(&self) -> EyreResult<()> {
        let res = self.post::<RebootDevice>().await;
        // server dies before responding to the reboot request
        assert!(res.is_err());
        RebootDevice.print_table()?;
        Ok(())
    }

    pub async fn show<T: ProcGet>(&self) -> EyreResult<()> {
        self.get::<T>().await?.print_table()?;
        Ok(())
    }

    pub async fn execute<T: ProcPost>(&self, params: T::Params) -> EyreResult<()> {
        self.post_with::<T>(params).await?.print_table()?;
        Ok(())
    }

    pub async fn system_status(&self) -> EyreResult<()> {
        let resp = self.get_text::<SystemStatus>().await?;

        // We have to use this trick to handle duplicate keys.
        // The system status cmd responds with duplicate keys in the
        // JSON response.
        let value: serde_json::Value = serde_json::from_str(&resp)?;
        let value: SystemStatus = serde_json::from_value(value)?;
        value.print_table()?;

        Ok(())
    }

    pub async fn delete_all_sms(&self) -> EyreResult<()> {
        let msg_ids = self
            .get::<SmsInbox>()
            .await?
            .messages
            .into_iter()
            .map(|m| m.id)
            .collect::<Vec<_>>();

        for msg_id in msg_ids {
            println!("Deleting {msg_id}...");
            self.post_with::<DeleteSms>(DeleteSmsParams { msg_id })
                .await?;
        }

        Ok(())
    }
}
