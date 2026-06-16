use clap::Parser;
use kimem::*;

#[tokio::main]
async fn main() -> EyreResult<()> {
    let args = Args::parse();
    let router = Router::new(&args.router, &args.username, &args.password)?;

    router.login().await?;
    match args.command {
        TopLevelCommands::Get { command } => match command {
            GetCommands::Devices => router.show::<StationList>().await?,
            GetCommands::Imei => router.show::<Imei>().await?,
            GetCommands::Imsi => router.show::<SimImsi>().await?,
            GetCommands::NetworkType => router.show::<NetworkType>().await?,
            GetCommands::Plmn => router.show::<SimPlmn>().await?,
            GetCommands::Rssi => router.show::<Rssi>().await?,
            GetCommands::Signalbar => router.show::<SignalBar>().await?,
            GetCommands::AirtimeBalance => router.show::<AirtimeBalance>().await?,
            GetCommands::WanIpaddr => router.show::<WanIpaddr>().await?,
            GetCommands::PppStatus => router.show::<PppStatus>().await?,
            GetCommands::CrVersion => router.show::<CrVersion>().await?,
            GetCommands::BatteryPercentage => router.show::<BatteryPercentage>().await?,
            GetCommands::BatteryExists => router.show::<BatteryExist>().await?,
        },
        TopLevelCommands::Post { command } => match command {
            PostCommands::Reboot => router.reboot().await.print_table(),
        },
    };
    router.logout().await?;

    Ok(())
}
