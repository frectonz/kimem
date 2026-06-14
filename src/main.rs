use clap::{Parser, Subcommand};
use std::net::IpAddr;

use kimem::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[clap(short, long, default_value = "192.168.0.1")]
    router: IpAddr,

    #[clap(short, long, default_value = "admin")]
    username: BoxStr,
    #[clap(short, long, default_value = "admin")]
    password: BoxStr,

    #[command(subcommand)]
    command: TopLevelCommands,
}

#[derive(Subcommand, Debug)]
enum TopLevelCommands {
    /// Commands for fetching data from the router.
    Get {
        #[command(subcommand)]
        command: GetCommands,
    },
    /// Commands for performing an action on the router.
    Post {
        #[command(subcommand)]
        command: PostCommands,
    },
}

#[derive(Subcommand, Debug)]
enum GetCommands {
    /// List connected devices.
    Devices,
    /// Fetch router IMEI.
    Imei,
    /// Fetch SIM IMSI.
    Imsi,
    /// Fetch the network type the router is connected to.
    NetworkType,
    /// Fetch SIM PLMN.
    Plmn,
    // Router signal strength in dBm.
    Rssi,
    // Router signal bar.
    Signalbar,
}

#[derive(Subcommand, Debug)]
enum PostCommands {
    /// Reboot the router.
    Reboot,
}

#[tokio::main]
async fn main() -> EyreResult<()> {
    let args = Args::parse();
    let router = Router::new(&args.router, &args.username, &args.password)?;

    router.login().await?;
    match args.command {
        TopLevelCommands::Get { command } => match command {
            GetCommands::Devices => {
                let station_list = router.execute_get::<StationList>().await?;
                station_list.print_table();
            }
            GetCommands::Imei => {
                let imei = router.execute_get::<Imei>().await?;
                imei.print_table();
            }
            GetCommands::Imsi => {
                let imsi = router.execute_get::<SimImsi>().await?;
                imsi.print_table();
            }
            GetCommands::NetworkType => {
                let network_type = router.execute_get::<NetworkType>().await?;
                network_type.print_table();
            }
            GetCommands::Plmn => {
                let sim_plmn = router.execute_get::<SimPlmn>().await?;
                sim_plmn.print_table();
            }
            GetCommands::Rssi => {
                let rssi = router.execute_get::<Rssi>().await?;
                rssi.print_table();
            }
            GetCommands::Signalbar => {
                let rssi = router.execute_get::<SignalBar>().await?;
                rssi.print_table();
            }
        },
        TopLevelCommands::Post { command } => match command {
            PostCommands::Reboot => {
                let reboot = router.reboot().await;
                reboot.print_table();
            }
        },
    }
    router.logout().await?;

    Ok(())
}
