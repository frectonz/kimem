use crate::common::*;
use crate::types::*;
use serde::Deserialize;

/// A read from `/reqproc/proc_get` that batches several `cmd`s in one
/// request via `multi_data=1`.
pub trait ProcGetMulti: serde::de::DeserializeOwned {
    const CMDS: &[&str];
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub msisdn: BoxStr,
    /// Cached USSD welcome text ("Welcome: 2517..."); on SIMs where
    /// `msisdn` is unprovisioned this is the only place the router
    /// knows the device's phone number.
    #[serde(deserialize_with = "de::ucs2")]
    pub old_sim_num: BoxStr,
    pub network_type: BoxStr,
    /// Operator name from the SIM, e.g. "Safaricom".
    pub sim_spn: BoxStr,
    pub sim_plmn: BoxStr,
    #[serde(deserialize_with = "de::from_str")]
    pub signalbar: u8,
    pub ppp_status: PppStatus,
    #[serde(deserialize_with = "de::from_str")]
    pub sms_unread_num: usize,
    #[serde(deserialize_with = "de::from_str")]
    pub battery_percentage: u8,
    #[serde(deserialize_with = "de::from_str")]
    pub sta_count: usize,
}

impl Info {
    fn phone_number(&self) -> Option<&str> {
        if !self.msisdn.is_empty() {
            return Some(&self.msisdn);
        }

        // The MSISDN in the USSD welcome text is the only long digit run;
        // menu item numbers are single digits.
        self.old_sim_num
            .split(|c: char| !c.is_ascii_digit())
            .find(|digits| digits.len() >= 9)
    }

    fn plmn(&self) -> BoxStr {
        if self.sim_spn.is_empty() {
            self.sim_plmn.clone()
        } else {
            format!("{} ({})", self.sim_spn, self.sim_plmn).into_boxed_str()
        }
    }
}

impl ProcGetMulti for Info {
    const CMDS: &[&str] = &[
        "msisdn",
        "old_sim_num",
        "network_type",
        "sim_spn",
        "sim_plmn",
        "signalbar",
        "ppp_status",
        "sms_unread_num",
        "battery_percentage",
        "sta_count",
    ];
}

impl Show for Info {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Info", "Value"])
            .add_row(["Phone Number", or_dash(self.phone_number().unwrap_or(""))])
            .add_row(["Network Type", &self.network_type])
            .add_row(["PLMN", &self.plmn()])
            .add_row(["Signal", &format!("{}/5", self.signalbar)])
            .add_row(["Internet", &self.ppp_status.to_string()])
            .add_row(["Unread SMS", &self.sms_unread_num.to_string()])
            .add_row(["Battery", &format!("{}%", self.battery_percentage)])
            .add_row(["Connected Devices", &self.sta_count.to_string()]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct System {
    pub mem_total: KibiBytes,
    pub mem_free: KibiBytes,
    pub mem_cached: KibiBytes,
    pub mem_active: KibiBytes,
    pub tz_cpu_usage: Percent,
    pub tz_flash_use: MegaBytes,
    pub tz_flash_total: MegaBytes,
}

impl ProcGetMulti for System {
    const CMDS: &[&str] = &[
        "mem_total",
        "mem_free",
        "mem_cached",
        "mem_active",
        "tz_cpu_usage",
        "tz_flash_use",
        "tz_flash_total",
    ];
}

impl Show for System {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["System", "Value"])
            .add_row(["Memory Total", &self.mem_total.to_string()])
            .add_row(["Memory Free", &self.mem_free.to_string()])
            .add_row(["Memory Cached", &self.mem_cached.to_string()])
            .add_row(["Memory Active", &self.mem_active.to_string()])
            .add_row(["CPU Usage", &self.tz_cpu_usage.to_string()])
            .add_row([
                "Flash Usage",
                &format!("{} / {}", self.tz_flash_use, self.tz_flash_total),
            ]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Internet {
    pub ppp_status: PppStatus,
    pub wan_ipaddr: BoxStr,
    #[serde(rename = "LocalDomain")]
    pub lan_domain: BoxStr,
    pub sim_imsi: BoxStr,
    pub realtime_rx_thrpt: BytesPerSecond,
    pub realtime_tx_thrpt: BytesPerSecond,
    pub realtime_rx_bytes: Bytes,
    pub realtime_tx_bytes: Bytes,
    pub monthly_rx_bytes: Bytes,
    pub monthly_tx_bytes: Bytes,
    pub monthly_time: Seconds,
}

impl ProcGetMulti for Internet {
    const CMDS: &[&str] = &[
        "ppp_status",
        "wan_ipaddr",
        "LocalDomain",
        "sim_imsi",
        "realtime_rx_thrpt",
        "realtime_tx_thrpt",
        "realtime_rx_bytes",
        "realtime_tx_bytes",
        "monthly_rx_bytes",
        "monthly_tx_bytes",
        "monthly_time",
    ];
}

impl Show for Internet {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Internet", "Value"])
            .add_row(["Status", &self.ppp_status.to_string()])
            .add_row(["WAN IP", or_dash(&self.wan_ipaddr)])
            .add_row(["LAN Domain", &self.lan_domain])
            .add_row(["IMSI", &self.sim_imsi])
            .add_row(["Download Speed", &self.realtime_rx_thrpt.to_string()])
            .add_row(["Upload Speed", &self.realtime_tx_thrpt.to_string()])
            .add_row(["Realtime RX", &self.realtime_rx_bytes.to_string()])
            .add_row(["Realtime TX", &self.realtime_tx_bytes.to_string()])
            .add_row(["Monthly RX", &self.monthly_rx_bytes.to_string()])
            .add_row(["Monthly TX", &self.monthly_tx_bytes.to_string()])
            .add_row(["Monthly Online", &self.monthly_time.to_string()]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Wifi {
    #[serde(rename = "SSID1")]
    pub ssid: BoxStr,
    #[serde(rename = "HideSSID", deserialize_with = "de::flag")]
    pub ssid_hidden: bool,
    pub wifi_mac: BoxStr,
    #[serde(rename = "MAX_Station_num", deserialize_with = "de::from_str")]
    pub max_station_num: u8,
    #[serde(rename = "Channel_cur", deserialize_with = "de::from_str")]
    pub channel: u8,
    #[serde(rename = "wifi_11n_cap")]
    pub channel_bandwidth: ChannelBandwidth,
    #[serde(rename = "AuthMode")]
    pub auth_mode: BoxStr,
    #[serde(rename = "WPAPSK1_encode", deserialize_with = "de::b64")]
    pub password: BoxStr,
    #[serde(rename = "m_SSID")]
    pub guest_ssid: BoxStr,
    #[serde(rename = "m_ssid_enable", deserialize_with = "de::flag")]
    pub guest_ssid_enabled: bool,
    #[serde(rename = "dhcpStart")]
    pub dhcp_start: BoxStr,
    #[serde(rename = "dhcpEnd")]
    pub dhcp_end: BoxStr,
}

impl Wifi {
    fn guest_ssid(&self) -> BoxStr {
        if self.guest_ssid.is_empty() {
            return "—".into();
        }

        let state = if self.guest_ssid_enabled {
            "Enabled"
        } else {
            "Disabled"
        };
        format!("{} ({state})", self.guest_ssid).into_boxed_str()
    }
}

impl ProcGetMulti for Wifi {
    const CMDS: &[&str] = &[
        "SSID1",
        "HideSSID",
        "wifi_mac",
        "MAX_Station_num",
        "Channel_cur",
        "wifi_11n_cap",
        "AuthMode",
        "WPAPSK1_encode",
        "m_SSID",
        "m_ssid_enable",
        "dhcpStart",
        "dhcpEnd",
    ];
}

impl Show for Wifi {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["WiFi", "Value"])
            .add_row(["SSID", &self.ssid])
            .add_row(["SSID Visible", yes_no(!self.ssid_hidden)])
            .add_row(["MAC Address", &self.wifi_mac])
            .add_row(["Max Stations", &self.max_station_num.to_string()])
            .add_row(["Channel", &self.channel.to_string()])
            .add_row(["Channel Bandwidth", &self.channel_bandwidth.to_string()])
            .add_row(["Auth Mode", &self.auth_mode])
            .add_row(["Password", &self.password])
            .add_row(["Guest SSID", &self.guest_ssid()])
            .add_row(["DHCP Start", &self.dhcp_start])
            .add_row(["DHCP End", &self.dhcp_end]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Power {
    #[serde(deserialize_with = "de::flag")]
    pub battery_exist: bool,
    #[serde(deserialize_with = "de::from_str")]
    pub battery_percentage: u8,
    #[serde(deserialize_with = "de::from_str")]
    pub battery_value: u8,
    #[serde(deserialize_with = "de::flag")]
    pub power_exist: bool,
}

impl ProcGetMulti for Power {
    const CMDS: &[&str] = &[
        "battery_exist",
        "battery_percentage",
        "battery_value",
        "power_exist",
    ];
}

impl Show for Power {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Power", "Value"])
            .add_row(["Battery Present", yes_no(self.battery_exist)])
            .add_row([
                "Battery Percentage",
                &format!("{}%", self.battery_percentage),
            ])
            .add_row(["Battery Power", &self.battery_value.to_string()])
            .add_row(["Connected To Power", yes_no(self.power_exist)]);

        println!("{table}");

        Ok(())
    }
}
