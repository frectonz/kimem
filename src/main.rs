use clap::Parser;
use kimem::*;

#[tokio::main]
async fn main() -> EyreResult<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let address = args.router_address();

    match args.command {
        TopLevelCommands::Zte { command } => {
            let router = zte::Router::new(&address, &args.username, &args.password, &args.format)?;
            run_zte(router, command).await
        }
        TopLevelCommands::Himi { command } => {
            let router =
                himi::Router::login(&address, &args.username, &args.password, &args.format).await?;
            run_himi(router, command).await
        }
    }
}

async fn run_himi(router: himi::Router, command: HimiCommands) -> EyreResult<()> {
    use himi::*;

    match command {
        HimiCommands::Get { command } => match command {
            HimiGetCommands::Overview => router.show::<Overview>().await?,
            HimiGetCommands::Status => router.show::<Status>().await?,
            HimiGetCommands::System => router.show::<SystemInfo>().await?,
            HimiGetCommands::Devices => router.show::<ConnectedDevices>().await?,
            HimiGetCommands::Wifi => router.show::<WifiInfo>().await?,
            HimiGetCommands::Apn => router.show::<ApnInfo>().await?,
            HimiGetCommands::Dhcp => router.show::<DhcpInfo>().await?,
            HimiGetCommands::Network => router.show_network().await?,
        },
        HimiCommands::Post { command } => match command {
            HimiPostCommands::Reboot => {
                return router.reboot().await;
            }
            HimiPostCommands::Wifi {
                state: Some(WifiPower::On),
                ..
            } => router.wifi_on().await?,
            HimiPostCommands::Wifi {
                state: None,
                changes,
                yes,
            } => {
                if router.edit_wifi(changes, yes).await? {
                    return Ok(());
                }
            }
        },
    }
    router.logout().await
}

async fn run_zte(router: zte::Router, command: ZteCommands) -> EyreResult<()> {
    use zte::*;

    router.login().await?;
    match command {
        ZteCommands::Get { command } => match command {
            GetCommands::Info => router.show_multi::<Info>().await?,
            GetCommands::System => router.show_multi::<System>().await?,
            GetCommands::Signal => router.show_signal().await?,
            GetCommands::Internet => router.show_internet().await?,
            GetCommands::Apn => router.show_apn().await?,
            GetCommands::Device => router.show_device().await?,
            GetCommands::Wifi => router.show_multi::<Wifi>().await?,
            GetCommands::Devices => router.show::<StationList>().await?,
            GetCommands::Sms { command: None } => router.show::<SmsInbox>().await?,
            GetCommands::Sms {
                command: Some(GetSmsCommands::Show { msg_id }),
            } => router.show_sms(msg_id).await?,
            GetCommands::Sms {
                command: Some(GetSmsCommands::Info),
            } => router.show_sms_info().await?,
            GetCommands::Syslog => router.show_syslog().await?,
            GetCommands::Airtime => router.show::<AirtimeBalance>().await?,
            GetCommands::Power => router.show_multi::<Power>().await?,
        },
        ZteCommands::Post { command } => match command {
            PostCommands::Reboot => {
                // The router goes down mid-reboot, so don't try to log out.
                return router.reboot().await;
            }
            PostCommands::Ussd { code } => router.ussd_session(&code).await?,
            PostCommands::Sms { command } => match command {
                PostSmsCommands::Send { number, message } => {
                    let params = SendSmsParams::new(&number, &message);
                    router.execute::<SendSms>(params).await?;
                }
                PostSmsCommands::Delete {
                    msg_id: MsgSelector::All,
                } => router.delete_all_sms().await?,
                PostSmsCommands::Delete {
                    msg_id: MsgSelector::Id(msg_id),
                } => {
                    let params = DeleteSmsParams {
                        msg_id: join_msg_ids([msg_id]),
                    };
                    router.execute::<DeleteSms>(params).await?;
                }
                PostSmsCommands::Mark {
                    msg_id: MsgSelector::All,
                } => router.mark_all_sms().await?,
                PostSmsCommands::Mark {
                    msg_id: MsgSelector::Id(msg_id),
                } => {
                    let params = MarkSmsParams::new(join_msg_ids([msg_id]));
                    router.execute::<MarkSms>(params).await?;
                }
            },
        },
    }
    router.logout().await
}
