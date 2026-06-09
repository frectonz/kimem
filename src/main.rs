use clap::Parser;
use std::net::IpAddr;

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
    dbg!(&args);
}
