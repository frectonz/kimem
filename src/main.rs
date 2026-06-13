use base64::Engine;
use clap::Parser;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::net::IpAddr;

use kimem::*;

#[derive(Parser, Debug)]
struct Args {
    #[clap(default_value = "192.168.0.1")]
    router: IpAddr,

    #[clap(default_value = "admin")]
    username: BoxStr,
    #[clap(default_value = "admin")]
    password: BoxStr,
}

#[tokio::main]
async fn main() -> EyreResult<()> {
    let args = Args::parse();
    let router = Router::new(args)?;

    let login = router.login().await?;
    dbg!(login);

    let station_list = router.execute_get::<StationList>().await?;
    dbg!(station_list);

    let network_type = router.execute_get::<NetworkType>().await?;
    dbg!(network_type);

    let plmn = router.execute_get::<SimPlmn>().await?;
    dbg!(plmn);

    let logout = router.logout().await?;
    dbg!(logout);

    let station_list_body = router.execute_get::<StationList>().await;
    assert!(station_list_body.is_err());

    Ok(())
}

struct Router {
    client: reqwest::Client,
    address: BoxStr,
    username: BoxStr,
    password: BoxStr,
}

fn b64(input: &str) -> BoxStr {
    base64::prelude::BASE64_STANDARD
        .encode(input)
        .into_boxed_str()
}

fn sha256(input: &str) -> BoxStr {
    let hash = sha2::Sha256::digest(input);
    hex::encode(hash).into_boxed_str()
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct LoginBody {
    result: BoxStr,
    power: BoxStr,
    unique_login_credentials: BoxStr,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ResultBody {
    result: BoxStr,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(tag = "isTest", rename = "false")]
struct ParamsBody<T: serde::Serialize> {
    cmd: BoxStr,
    #[serde(flatten)]
    payload: T,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(tag = "isTest", rename = "false")]
struct FormBody<T: serde::Serialize> {
    #[serde(rename = "goformId")]
    goform_id: BoxStr,
    #[serde(flatten)]
    payload: T,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct LoginFormBody {
    username: BoxStr,
    password: BoxStr,
    unique_login_credentials: BoxStr,
}

impl Router {
    fn new(args: Args) -> EyreResult<Self> {
        let router = args.router;
        Ok(Self {
            client: reqwest::Client::builder().user_agent("Kimem CLI").build()?,
            address: format!("http://{router}").into_boxed_str(),
            username: args.username,
            password: args.password,
        })
    }

    async fn execute_get_with<Req: ProcGet>(
        &self,
        params: Req::Params,
    ) -> EyreResult<Req::Response> {
        let address = self.address.as_ref();

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

    async fn execute_get<Req: ProcGet>(&self) -> EyreResult<Req::Response> {
        self.execute_get_with::<Req>(Req::Params::default()).await
    }

    async fn execute_post_with<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        gofrom_id: &str,
        body: B,
    ) -> EyreResult<T> {
        let address = self.address.as_ref();
        let url = format!("{address}/reqproc/proc_post");

        let form = FormBody {
            goform_id: gofrom_id.into(),
            payload: body,
        };

        let body = self
            .client
            .post(url)
            .form(&form)
            .header("Referer", self.address.as_ref())
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(body)
    }

    async fn execute_post<T: serde::de::DeserializeOwned>(&self, gofrom_id: &str) -> EyreResult<T> {
        self.execute_post_with(gofrom_id, ()).await
    }

    async fn login(&self) -> EyreResult<LoginBody> {
        let password = self.password.as_ref();
        let random_login = self.execute_get::<GetRandomLogin>().await?;
        let nonce = random_login.random_login;

        let form = LoginFormBody {
            username: b64(&self.username),
            password: b64(&sha256(&format!("{nonce}{password}"))),
            unique_login_credentials: "1".into(),
        };

        let body: LoginBody = self.execute_post_with("LOGIN", form).await?;

        Ok(body)
    }

    async fn logout(&self) -> EyreResult<ResultBody> {
        self.execute_post("LOGOUT").await
    }
}
