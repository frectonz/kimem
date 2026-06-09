use clap::Parser;
use serde::Deserialize;
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

    let nonce = router.fetch_nonce().await;
    dbg!(nonce);
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
}
