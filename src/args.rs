use crate::common::*;
use clap::{Parser, Subcommand};
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[command(version, about)]
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
pub enum GetCommands {
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
    /// Fetch router RSCP in dBm.
    Rscp,
    /// Fetch LTE RSRP.
    LteRsrp,
    /// Fetch LTE RSRQ.
    LteRsrq,
    /// Fetch Cell ID.
    LteCellId,
    /// Fetch eNodeB ID.
    LteEnodebId,
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
    /// Fetch battery percentage.
    BatteryPercentage,
    /// Check if batttery exists.
    BatteryExists,
    /// Check if power exists.
    PowerExists,
    /// Fetch messages in SMS inbox.
    Sms,
}

#[derive(Subcommand, Debug)]
pub enum PostCommands {
    /// Reboot the router.
    Reboot,
    /// Delete message from inbox.
    DeleteSms { msg_id: BoxStr },
    /// Delete all message from inbox.
    DeleteAllSms,
}
