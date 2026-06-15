use clap::{Parser, Subcommand};
use kimem::*;
use std::net::IpAddr;

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
    /// Fetch the network type.
    NetworkType,
    /// Fetch SIM PLMN.
    Plmn,
    /// Fetch router signal strength in dBm.
    Rssi,
    /// Fetch router signal bar.
    Signalbar,
    /// Fetch last cached airtime balance.
    AirtimeBalance,
    /// Fetch WAN IP address.
    WanIpaddr,
    /// Fetch PPP status.
    PppStatus,
    /// Fetch firmware version.
    CrVersion,
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
                let station_list = router.get::<StationList>().await?;
                station_list.print_table();
            }
            GetCommands::Imei => {
                let imei = router.get::<Imei>().await?;
                imei.print_table();
            }
            GetCommands::Imsi => {
                let imsi = router.get::<SimImsi>().await?;
                imsi.print_table();
            }
            GetCommands::NetworkType => {
                let network_type = router.get::<NetworkType>().await?;
                network_type.print_table();
            }
            GetCommands::Plmn => {
                let sim_plmn = router.get::<SimPlmn>().await?;
                sim_plmn.print_table();
            }
            GetCommands::Rssi => {
                let rssi = router.get::<Rssi>().await?;
                rssi.print_table();
            }
            GetCommands::Signalbar => {
                let rssi = router.get::<SignalBar>().await?;
                rssi.print_table();
            }
            GetCommands::AirtimeBalance => {
                let airtime_balance = router.get::<AirtimeBalance>().await?;
                airtime_balance.print_table();
            }
            GetCommands::WanIpaddr => {
                let wan_ipaddr = router.get::<WanIpaddr>().await?;
                wan_ipaddr.print_table();
            }
            GetCommands::PppStatus => {
                let ppp_status = router.get::<PppStatus>().await?;
                ppp_status.print_table();
            }
            GetCommands::CrVersion => {
                let cr_version = router.get::<CrVersion>().await?;
                cr_version.print_table();
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
