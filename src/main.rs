use base64::Engine;
use clap::Parser;
use serde::Deserialize;
use sha2::Digest;
use std::net::IpAddr;

type BoxStr = Box<str>;

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
async fn main() {
    let args = Args::parse();
    let router: Router = args.into();

    let login = router.login().await;
    dbg!(login.result);
    dbg!(login.power);
    dbg!(login.unique_login_credentials);
}

struct Router {
    client: reqwest::Client,
    address: String,
    username: String,
    password: String,
}

impl From<Args> for Router {
    fn from(args: Args) -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Kimem CLI")
                .build()
                .unwrap(),
            address: format!("http://{}", args.router),
            username: args.username,
            password: args.password,
        }
    }
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

impl Router {
    async fn fetch_nonce(&self) -> BoxStr {
        #[derive(Deserialize)]
        struct NonceBody {
            random_login: BoxStr,
        }

        let url = format!(
            "{}/reqproc/proc_get?cmd=get_random_login&isTest=false",
            self.address
        );

        let nonce = self
            .client
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<NonceBody>()
            .await
            .unwrap();

        nonce.random_login
    }

    async fn login(&self) -> LoginBody {
        let nonce = self.fetch_nonce().await;
        let url = format!("{}/reqproc/proc_post", self.address);

        self.client
            .post(url)
            .form(&[
                ("username", b64(&self.username).as_str()),
                (
                    "password",
                    &b64(&sha256(&format!("{nonce}{}", self.password))),
                ),
                ("goformId", "LOGIN"),
                ("unique_login_credentials", "1"),
                ("isTest", "false"),
            ])
            .header("Referer", self.address.clone())
            .send()
            .await
            .unwrap()
            .json::<LoginBody>()
            .await
            .unwrap()
    }
}
