use base64::Engine;
use clap::Parser;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::net::IpAddr;

type BoxStr = Box<str>;
type EyreResult<T> = color_eyre::Result<T>;

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

    let station_list_body = router.fetch_connected_devices().await?;
    dbg!(station_list_body);

    let imei = router.fetch_imei().await?;
    dbg!(imei);

    let imsi = router.fetch_sim_imsi().await?;
    dbg!(imsi);

    let network_type = router.fetch_network_type().await?;
    dbg!(network_type);

    Ok(())
}

struct Router {
    client: reqwest::Client,
    address: BoxStr,
    username: BoxStr,
    password: BoxStr,
}

fn b64(input: &str) -> String {
    base64::prelude::BASE64_STANDARD.encode(input)
}

fn sha256(input: &str) -> String {
    let hash = sha2::Sha256::digest(input);
    hex::encode(hash)
}

#[derive(Debug, Deserialize)]
struct NonceBody {
    random_login: BoxStr,
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
struct StationListBody {
    station_list: Vec<ConnectedDevice>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ConnectedDevice {
    connect_time: BoxStr,
    ssid_index: BoxStr,
    dev_type: BoxStr,
    mac_addr: BoxStr,
    hostname: BoxStr,
    ip_addr: BoxStr,
    ipv6: BoxStr,
    ipv6_local: BoxStr,
    ip_type: BoxStr,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ImeiBody {
    imei: BoxStr,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct SimImsiBody {
    sim_imsi: BoxStr,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct NetworkTypeBody {
    network_type: BoxStr,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct FormBody<T: serde::Serialize> {
    #[serde(rename = "goformId")]
    goform_id: BoxStr,
    #[serde(rename = "isTest")]
    is_test: BoxStr,

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

    async fn execute_get<T: serde::de::DeserializeOwned>(&self, cmd: &str) -> EyreResult<T> {
        let address = self.address.as_ref();
        let url = format!("{address}/reqproc/proc_get?cmd={cmd}&isTest=false");
        let body = self.client.get(url).send().await?.json::<T>().await?;
        Ok(body)
    }

    #[allow(dead_code)]
    async fn execute_get_return_txt(&self, cmd: &str) -> EyreResult<String> {
        let address = self.address.as_ref();
        let url = format!("{address}/reqproc/proc_get?cmd={cmd}&isTest=false");
        let body = self.client.get(url).send().await?.text().await?;
        Ok(body)
    }

    async fn fetch_nonce(&self) -> EyreResult<BoxStr> {
        let body = self.execute_get::<NonceBody>("get_random_login").await?;
        Ok(body.random_login)
    }

    async fn fetch_connected_devices(&self) -> EyreResult<StationListBody> {
        let body = self.execute_get::<StationListBody>("station_list").await?;
        Ok(body)
    }

    async fn fetch_imei(&self) -> EyreResult<ImeiBody> {
        let body = self.execute_get::<ImeiBody>("imei").await?;
        Ok(body)
    }

    async fn fetch_sim_imsi(&self) -> EyreResult<SimImsiBody> {
        let body = self.execute_get::<SimImsiBody>("sim_imsi").await?;
        Ok(body)
    }

    async fn fetch_network_type(&self) -> EyreResult<NetworkTypeBody> {
        let body = self.execute_get::<NetworkTypeBody>("network_type").await?;
        Ok(body)
    }

    async fn execute_post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        gofrom_id: &str,
        body: B,
    ) -> EyreResult<T> {
        let address = self.address.as_ref();
        let url = format!("{address}/reqproc/proc_post");

        let form = FormBody {
            goform_id: gofrom_id.into(),
            is_test: "false".into(),
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

    async fn login(&self) -> EyreResult<LoginBody> {
        let password = self.password.as_ref();
        let nonce = self.fetch_nonce().await?;

        let form = LoginFormBody {
            username: b64(&self.username).into(),
            password: b64(&sha256(&format!("{nonce}{password}"))).into_boxed_str(),
            unique_login_credentials: "1".into(),
        };

        let body: LoginBody = self.execute_post("LOGIN", form).await?;

        Ok(body)
    }
}
