use clap::Parser;
use kimem::*;

#[tokio::main]
async fn main() -> EyreResult<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let router = Router::new(&args.router, &args.username, &args.password)?;

    router.login().await?;
    match args.command {
        TopLevelCommands::Get { command } => match command {
            GetCommands::Info => router.show_multi::<Info>().await?,
            GetCommands::System => router.show_multi::<System>().await?,
            GetCommands::Signal => router.show_signal().await?,
            GetCommands::Internet => router.show_multi::<Internet>().await?,
            GetCommands::Device => router.show_device().await?,
            GetCommands::Wifi => router.show_multi::<Wifi>().await?,
            GetCommands::Devices => router.show::<StationList>().await?,
            GetCommands::Sms { command: None } => router.show::<SmsInbox>().await?,
            GetCommands::Sms {
                command: Some(GetSmsCommands::Show { msg_id }),
            } => router.show_sms(msg_id).await?,
            GetCommands::Syslog => router.show_syslog().await?,
            GetCommands::Airtime => router.show::<AirtimeBalance>().await?,
            GetCommands::Power => router.show_multi::<Power>().await?,
        },
        TopLevelCommands::Post { command } => match command {
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
    router.logout().await?;

    Ok(())
}
