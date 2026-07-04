use crate::common::*;
use clap::{Parser, Subcommand};
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[command(version, about, flatten_help = true, disable_help_subcommand = true)]
pub struct Args {
    #[clap(short, long, default_value = "192.168.0.1")]
    pub router: IpAddr,

    #[clap(short, long, default_value = "admin")]
    pub username: BoxStr,
    #[clap(short, long, default_value = "admin")]
    pub password: BoxStr,

    #[command(subcommand)]
    pub command: TopLevelCommands,
}

#[derive(Subcommand, Debug)]
pub enum TopLevelCommands {
    /// Commands for fetching data from the router.
    #[command(flatten_help = true)]
    Get {
        #[command(subcommand)]
        command: GetCommands,
    },
    /// Commands for performing an action on the router.
    #[command(flatten_help = true)]
    Post {
        #[command(subcommand)]
        command: PostCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum GetCommands {
    /// Router status information.
    Info,
    /// Router system information.
    System,
    /// Router connection signal information.
    Signal,
    /// Router internet connection information.
    Internet,
    /// Router device information.
    Device,
    /// Router wifi config information.
    Wifi,
    /// List connected devices.
    Devices,
    /// List SMS messages.
    #[command(flatten_help = true)]
    Sms {
        #[command(subcommand)]
        command: Option<GetSmsCommands>,
    },
    /// Show router system logs.
    Syslog,
    /// Last cached airtime balance.
    Airtime,
    /// Router power information.
    Power,
}

#[derive(Subcommand, Debug)]
pub enum GetSmsCommands {
    /// Show full SMS message.
    Show {
        /// ID of the message to show.
        msg_id: usize,
    },
}

#[derive(Subcommand, Debug)]
pub enum PostCommands {
    /// Reboot the router.
    Reboot,
    /// SMS actions.
    #[command(flatten_help = true)]
    Sms {
        #[command(subcommand)]
        command: PostSmsCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum PostSmsCommands {
    /// Send an SMS message.
    Send {
        /// Recipient phone number.
        number: BoxStr,
        /// Message text.
        message: BoxStr,
    },
    /// Delete a message (or "all") from the inbox.
    Delete {
        /// ID of the message to delete, or "all".
        #[arg(value_parser = parse_msg_selector)]
        msg_id: MsgSelector,
    },
    /// Mark a message (or "all") as read.
    Mark {
        /// ID of the message to mark, or "all".
        #[arg(value_parser = parse_msg_selector)]
        msg_id: MsgSelector,
    },
}

/// A message ID argument that also accepts the literal "all".
#[derive(Debug, Clone, Copy)]
pub enum MsgSelector {
    All,
    Id(usize),
}

fn parse_msg_selector(raw: &str) -> Result<MsgSelector, String> {
    if raw.eq_ignore_ascii_case("all") {
        return Ok(MsgSelector::All);
    }

    raw.parse()
        .map(MsgSelector::Id)
        .map_err(|_| format!("expected a message ID or \"all\", got {raw:?}"))
}
