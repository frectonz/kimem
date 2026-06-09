use base64::Engine;
use clap::Parser;
use serde::Deserialize;
use sha2::Digest;
use std::net::IpAddr;

type BoxStr = Box<str>;
type EyreResult<T> = color_eyre::Result<T>;

#[derive(Parser, Debug)]
struct Args {
    #[clap(default_value = "192.168.0.1")]
    router: IpAddr,

    #[clap(default_value = "admin")]
    username: String,
    #[clap(default_value = "admin")]
    password: String,
}

#[tokio::main]
async fn main() -> EyreResult<()> {
    let args = Args::parse();
    let router = Router::new(args)?;

    let login = router.login().await?;
    dbg!(login.result);
    dbg!(login.power);
    dbg!(login.unique_login_credentials);

    Ok(())
}

struct Router {
    client: reqwest::Client,
    address: String,
    username: String,
    password: String,
}

fn b64(input: &str) -> String {
    base64::prelude::BASE64_STANDARD.encode(input)
}

fn sha256(input: &str) -> String {
    let hash = sha2::Sha256::digest(input);
    hex::encode(hash)
}

#[derive(Debug, Deserialize)]
struct LoginBody {
    result: BoxStr,
    power: BoxStr,
    unique_login_credentials: BoxStr,
}

#[derive(Deserialize)]
struct NonceBody {
    random_login: BoxStr,
}

impl Router {
    fn new(args: Args) -> EyreResult<Self> {
        let router = args.router;
        Ok(Self {
            client: reqwest::Client::builder().user_agent("Kimem CLI").build()?,
            address: format!("http://{router}"),
            username: args.username,
            password: args.password,
        })
    }

    async fn fetch_nonce(&self) -> EyreResult<BoxStr> {
        let address = self.address.as_str();
        let url = format!("{address}/reqproc/proc_get?cmd=get_random_login&isTest=false");

        let nonce = self
            .client
            .get(url)
            .send()
            .await?
            .json::<NonceBody>()
            .await?;

        Ok(nonce.random_login)
    }

    async fn login(&self) -> EyreResult<LoginBody> {
        let address = self.address.as_str();
        let url = format!("{address}/reqproc/proc_post");

        let password = self.password.as_str();
        let nonce = self.fetch_nonce().await?;

        let body = self
            .client
            .post(url)
            .form(&[
                ("username", b64(&self.username).as_str()),
                ("password", &b64(&sha256(&format!("{nonce}{password}")))),
                ("goformId", "LOGIN"),
                ("unique_login_credentials", "1"),
                ("isTest", "false"),
            ])
            .header("Referer", self.address.clone())
            .send()
            .await?
            .json::<LoginBody>()
            .await?;

        Ok(body)
    }
}
