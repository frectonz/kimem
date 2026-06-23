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
            GetCommands::Rscp => router.show::<Rscp>().await?,
            GetCommands::Signalbar => router.show::<SignalBar>().await?,
            GetCommands::AirtimeBalance => router.show::<AirtimeBalance>().await?,
            GetCommands::WanIpaddr => router.show::<WanIpaddr>().await?,
            GetCommands::PppStatus => router.show::<PppStatus>().await?,
            GetCommands::CrVersion => router.show::<CrVersion>().await?,
            GetCommands::BatteryPercentage => router.show::<BatteryPercentage>().await?,
            GetCommands::BatteryExists => router.show::<BatteryExist>().await?,
            GetCommands::PowerExists => router.show::<PowerExist>().await?,
            GetCommands::Sms => router.show::<SmsInbox>().await?,
            GetCommands::LteRsrp => router.show::<LteRsrp>().await?,
            GetCommands::LteRsrq => router.show::<LteRsrq>().await?,
            GetCommands::LteCellId => router.show::<LteCellId>().await?,
            GetCommands::LteEnodebId => router.show::<LteEnodebId>().await?,
            GetCommands::LteTac => router.show::<LteTac>().await?,
            GetCommands::NvArfcn => router.show::<NvArfcn>().await?,
            GetCommands::LanIpaddr => router.show::<LanIpaddr>().await?,
            GetCommands::LocalDomain => router.show::<LocalDomain>().await?,
            GetCommands::DhcpStart => router.show::<DhcpStart>().await?,
            GetCommands::DhcpEnd => router.show::<DhcpEnd>().await?,
            GetCommands::PrimarySsid => router.show::<PrimarySsid>().await?,
            GetCommands::SecondarySsid => router.show::<SecondarySsid>().await?,
            GetCommands::PrimarySsidPsk => router.show::<PrimarySsidPsk>().await?,
            GetCommands::SecondarySsidPsk => router.show::<SecondarySsidPsk>().await?,
            GetCommands::PrimarySsidAuthMode => router.show::<PrimarySsidAuthMode>().await?,
            GetCommands::SecondarySsidAuthMode => router.show::<SecondarySsidAuthMode>().await?,
            GetCommands::MonthlyRx => router.show::<MonthlyRx>().await?,
            GetCommands::MonthlyTx => router.show::<MonthlyTx>().await?,
            GetCommands::RealtimeRx => router.show::<RealtimeRx>().await?,
            GetCommands::RealtimeTx => router.show::<RealtimeTx>().await?,
            GetCommands::HardwareVersion => router.show::<HardwareVersion>().await?,
            GetCommands::SystemStatus => router.system_status().await?,
            GetCommands::MaxStationNum => router.show::<MaxStationNum>().await?,
            GetCommands::DeviceVersion => router.show::<DeviceVersion>().await?,
            GetCommands::SmsParameterInfo => router.show::<SmsParameterInfo>().await?,
        },
        TopLevelCommands::Post { command } => match command {
            PostCommands::Reboot => router.reboot().await?,
            PostCommands::DeleteSms { msg_id } => {
                let params = DeleteSmsParams { msg_id };
                router.execute::<DeleteSms>(params).await?
            }
            PostCommands::DeleteAllSms => router.delete_all_sms().await?,
            PostCommands::SendMessage { number, message } => {
                let params = SendSmsParams::new(&number, &message);
                router.execute::<SendSms>(params).await?
            }
            PostCommands::MarkSms { msg_id } => {
                let params = MarkSmsParams::new(msg_id);
                router.execute::<MarkSms>(params).await?
            }
        },
    };
    router.logout().await?;

    Ok(())
}
