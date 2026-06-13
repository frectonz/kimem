use clap::Parser;
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
    let router = Router::new(&args.router, &args.username, &args.password)?;

    let login = router.login().await?;
    login.print_table();

    let station_list = router.execute_get::<StationList>().await?;
    station_list.print_table();

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
