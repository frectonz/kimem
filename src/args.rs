use crate::common::*;
use clap::{Parser, Subcommand};
use color_eyre::eyre;
use std::{net::IpAddr, str::FromStr};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Router address [default: 192.168.0.1 for zte, 192.168.100.1 for himi]
    #[clap(short, long)]
    pub router: Option<IpAddr>,

    #[clap(short, long, default_value = "admin")]
    pub username: BoxStr,
    #[clap(short, long, default_value = "admin")]
    pub password: BoxStr,

    #[clap(short, long, default_value = "table")]
    pub format: Format,

    #[command(subcommand)]
    pub command: TopLevelCommands,
}

#[derive(Debug, Clone)]
pub enum Format {
    /// Print output as JSON.
    Json,
    /// Print output as table.
    Table,
}

#[derive(Subcommand, Debug)]
pub enum TopLevelCommands {
    /// ZTE-firmware MIFIs (e.g. Safaricom Kimem).
    Zte {
        #[command(subcommand)]
        command: ZteCommands,
    },
    /// HiMILink-firmware MIFIs ("4G LTE" web UI).
    Himi {
        #[command(subcommand)]
        command: HimiCommands,
    },
}

impl Args {
    pub fn router_address(&self) -> IpAddr {
        let default = match self.command {
            TopLevelCommands::Zte { .. } => [192, 168, 0, 1],
            TopLevelCommands::Himi { .. } => [192, 168, 100, 1],
        };

        self.router.unwrap_or(IpAddr::from(default))
    }
}

#[derive(Subcommand, Debug)]
pub enum HimiCommands {
    /// Commands for fetching data from the router.
    Get {
        #[command(subcommand)]
        command: HimiGetCommands,
    },
    /// Commands for performing an action on the router.
    Post {
        #[command(subcommand)]
        command: HimiPostCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum HimiGetCommands {
    /// Firmware, hardware and SIM identifiers.
    Overview,
    /// Carrier, signal, internet, SIM and Wi-Fi state.
    Status,
    /// System time, uptime, CPU temperature, battery and cell.
    System,
    /// List connected devices.
    Devices,
    /// Wi-Fi access point configuration.
    Wifi,
    /// APN profiles and which one is active.
    Apn,
    /// LAN DHCP server configuration.
    Dhcp,
    /// Network mode, active SIM, USB tethering and connection check.
    Network,
}

#[derive(Subcommand, Debug)]
pub enum HimiPostCommands {
    /// Reboot the router.
    Reboot,
    /// Edit Wi-Fi settings, or turn Wi-Fi back on.
    ///
    /// Without arguments it prompts for every setting. With setting flags it
    /// changes only those and keeps the rest.
    Wifi {
        #[arg(conflicts_with = "WifiChanges")]
        state: Option<WifiPower>,
        #[command(flatten)]
        changes: WifiChanges,
        /// Skip the confirmation prompt.
        #[arg(short, long)]
        yes: bool,
    },
    /// Change the admin password.
    Password {
        /// The new admin password. Prompted for when omitted.
        #[arg(long)]
        new_password: Option<String>,
    },
    /// Choose which networks the modem may use.
    NetworkMode {
        /// Prompted for when omitted.
        mode: Option<NetworkModeArg>,
    },
    /// Turn USB tethering on or off.
    Usb { state: Toggle },
}

#[derive(clap::Args, Debug)]
pub struct WifiChanges {
    /// Network name, 1 to 64 characters.
    #[arg(long)]
    pub ssid: Option<String>,
    #[arg(long)]
    pub security: Option<WifiSecurityArg>,
    /// Wi-Fi password, 8 to 32 characters.
    #[arg(long)]
    pub passphrase: Option<String>,
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=13))]
    pub channel: Option<u8>,
    /// Whether the network name is broadcast.
    #[arg(long)]
    pub broadcast: Option<bool>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum Toggle {
    On,
    Off,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum WifiPower {
    On,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum WifiSecurityArg {
    None,
    WpaPsk,
    Wpa2Psk,
}

impl WifiSecurityArg {
    pub fn code(self) -> &'static str {
        match self {
            Self::None => "0",
            Self::WpaPsk => "1",
            Self::Wpa2Psk => "4",
        }
    }
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum NetworkModeArg {
    /// 4G with 3G fallback.
    #[value(name = "4g-3g")]
    Auto,
    #[value(name = "4g")]
    Lte,
    #[value(name = "3g")]
    Wcdma,
}

impl NetworkModeArg {
    pub fn code(self) -> &'static str {
        match self {
            Self::Auto => "0",
            Self::Lte => "1",
            Self::Wcdma => "2",
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum ZteCommands {
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
    /// Router status information.
    Info,
    /// Router system information.
    System,
    /// Router connection signal information.
    Signal,
    /// Router internet connection information.
    Internet,
    /// Active APN (access point) configuration.
    Apn,
    /// Router device information.
    Device,
    /// Router wifi config information.
    Wifi,
    /// List connected devices.
    Devices,
    /// List SMS messages.
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
    /// Show SMS storage usage and settings.
    Info,
}

#[derive(Subcommand, Debug)]
pub enum PostCommands {
    /// Reboot the router.
    Reboot,
    /// Start an interactive USSD session.
    Ussd {
        /// USSD code (e.g. '*704#') or a built-in name.
        ///
        /// Built-in names: menu (*777#), balance (*704#),
        /// bundles (*777*02#), mpesa (*733#).
        #[arg(value_parser = parse_ussd_code)]
        code: BoxStr,
    },
    /// SMS actions.
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

/// Built-in Safaricom Ethiopia USSD codes, dialable by name. "balance"
/// and "bundles" are what the router dashboard's own buttons dial.
const USSD_CODES: &[(&str, &str)] = &[
    ("menu", "*777#"),
    ("balance", "*704#"),
    ("bundles", "*777*02#"),
    ("mpesa", "*733#"),
];

fn parse_ussd_code(raw: &str) -> Result<BoxStr, String> {
    if raw.contains('*') || raw.contains('#') {
        return Ok(raw.into());
    }

    USSD_CODES
        .iter()
        .find(|(name, _)| raw.eq_ignore_ascii_case(name))
        .map(|(_, code)| (*code).into())
        .ok_or_else(|| {
            let names: BoxList<String> = USSD_CODES
                .iter()
                .map(|(name, code)| format!("{name} ({code})"))
                .collect();
            format!(
                "expected a USSD code or a built-in name, got {raw:?}; built-ins: {}",
                names.join(", ")
            )
        })
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

impl FromStr for Format {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "table" => Ok(Self::Table),
            _ => eyre::bail!("unknow format type: {s}"),
        }
    }
}
