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
