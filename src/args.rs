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
    /// Fetch Tracking Area Code.
    LteTac,
    /// Fetch router signal bar.
    Signalbar,
    /// Fetch last cached airtime balance.
    AirtimeBalance,
    /// Fetch WAN IP address.
    WanIpaddr,
    /// Fetch PPP status.
    PppStatus,
    /// Fetch LAN IP address.
    LanIpaddr,
    /// Fetch local domain.
    LocalDomain,
    /// Fetch DHCP start.
    DhcpStart,
    /// Fetch DHCP end.
    DhcpEnd,
    /// Fetch battery percentage.
    BatteryPercentage,
    /// Check if batttery exists.
    BatteryExists,
    /// Check if power exists.
    PowerExists,
    /// Fetch messages in SMS inbox.
    Sms,
    /// Fetch NV ARFCN.
    NvArfcn,
    /// Fetch primary SSID.
    PrimarySsid,
    /// Fetch secondary SSID.
    SecondarySsid,
    /// Fetch primary SSID's password.
    PrimarySsidPsk,
    /// Fetch secondary SSID's password.
    SecondarySsidPsk,
    /// Fetch primary SSID's auth mode.
    PrimarySsidAuthMode,
    /// Fetch secondary SSID's auth mode.
    SecondarySsidAuthMode,
    /// Fetch monthly RX bytes.
    MonthlyRx,
    /// Fetch monthly TX bytes.
    MonthlyTx,
    /// Fetch realtime RX bytes.
    RealtimeRx,
    /// Fetch realtime TX bytes.
    RealtimeTx,
    /// Fetch firmware version.
    CrVersion,
    /// Fetch hardware version.
    HardwareVersion,
    /// Fetch system status.
    SystemStatus,
    /// Fetch max station num.
    MaxStationNum,
    /// Fetch device version.
    DeviceVersion,
    SmsParameterInfo,
}

#[derive(Subcommand, Debug)]
pub enum PostCommands {
    /// Reboot the router.
    Reboot,
    /// Delete message from inbox.
    DeleteSms { msg_id: BoxStr },
    /// Delete all message from inbox.
    DeleteAllSms,
    /// Send an SMS message.
    SendMessage { number: BoxStr, message: BoxStr },
    /// Mark SMS message as read.
    MarkSms { msg_id: BoxStr },
}
