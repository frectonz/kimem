use std::net::IpAddr;

use serde::Serialize;

use crate::*;

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

    pub async fn execute_get_with<Req: ProcGet>(
        &self,
        params: Req::Params,
    ) -> EyreResult<Req::Response> {
        let address = self.address.as_ref();

        #[derive(Debug, Serialize)]
        #[serde(tag = "isTest", rename = "false")]
        struct ParamsBody<T: serde::Serialize> {
            cmd: BoxStr,
            #[serde(flatten)]
            payload: T,
        }

        let params = serde_urlencoded::to_string(&ParamsBody {
            cmd: Req::CMD.into(),
            payload: params,
        })?;

        let url = format!("{address}/reqproc/proc_get?{params}");
        let body = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Req::Response>()
            .await?;

        Ok(body)
    }

    pub async fn execute_get<Req: ProcGet>(&self) -> EyreResult<Req::Response> {
        self.execute_get_with::<Req>(Req::Params::default()).await
    }

    pub async fn execute_post_with<Req: ProcPost>(
        &self,
        params: Req::Params,
    ) -> EyreResult<Req::Response> {
        let address = self.address.as_ref();
        let url = format!("{address}/reqproc/proc_post");

        #[derive(Debug, Serialize)]
        #[serde(tag = "isTest", rename = "false")]
        struct FormBody<T: serde::Serialize> {
            #[serde(rename = "goformId")]
            goform_id: BoxStr,
            #[serde(flatten)]
            payload: T,
        }

        let form = FormBody {
            goform_id: Req::GOFROM_ID.into(),
            payload: params,
        };

        let body = self
            .client
            .post(url)
            .form(&form)
            .header("Referer", self.address.as_ref())
            .send()
            .await?
            .json::<Req::Response>()
            .await?;

        Ok(body)
    }

    pub async fn execute_post<Req: ProcPost>(&self) -> EyreResult<Req::Response> {
        self.execute_post_with::<Req>(Req::Params::default()).await
    }

    pub async fn login(&self) -> EyreResult<Login> {
        let password = self.password.as_ref();
        let random_login = self.execute_get::<GetRandomLogin>().await?;
        let nonce = random_login.random_login;

        let form = LoginParams {
            username: b64(&self.username),
            password: b64(&sha256(&format!("{nonce}{password}"))),
            unique_login_credentials: "1".into(),
        };

        let body = self.execute_post_with::<Login>(form).await?;

        Ok(body)
    }

    pub async fn logout(&self) -> EyreResult<Logout> {
        self.execute_post::<Logout>().await
    }

    pub async fn reboot(&self) -> RebootDevice {
        let res = self.execute_post::<RebootDevice>().await;
        // server dies before responding to the reboot request
        assert!(res.is_err());
        RebootDevice
    }
}
