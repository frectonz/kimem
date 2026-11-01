use crate::common::*;
use crate::types::{Seconds, de::flag};
use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use std::fmt;

pub trait HimiGet: serde::de::DeserializeOwned {
    const CMDID: &str;
}

fn int_or_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr + Deserialize<'de>,
    T::Err: fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Raw<'a, T> {
        Num(T),
        Str(std::borrow::Cow<'a, str>),
    }

    match Raw::<T>::deserialize(deserializer)? {
        Raw::Num(value) => Ok(value),
        Raw::Str(raw) => raw
            .trim()
            .parse()
            .map_err(|e| D::Error::custom(format!("cannot parse {raw:?}: {e}"))),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum WifiState {
    #[serde(rename(deserialize = "open"))]
    On,
    #[serde(rename(deserialize = "close"))]
    Off,
    #[serde(untagged)]
    Unknown(BoxStr),
}

impl fmt::Display for WifiState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::On => write!(f, "On"),
            Self::Off => write!(f, "Off"),
            Self::Unknown(raw) => write!(f, "Unknown ({raw})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum WifiSecurity {
    #[serde(rename(deserialize = "0"))]
    Open,
    #[serde(rename(deserialize = "1"))]
    WpaPsk,
    #[serde(rename(deserialize = "4"))]
    Wpa2Psk,
    #[serde(untagged)]
    Unknown(BoxStr),
}

impl fmt::Display for WifiSecurity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "None"),
            Self::WpaPsk => write!(f, "WPA-PSK"),
            Self::Wpa2Psk => write!(f, "WPA2-PSK"),
            Self::Unknown(raw) => write!(f, "Unknown ({raw})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum ApnAuth {
    #[serde(rename(deserialize = "0"))]
    None,
    #[serde(rename(deserialize = "1"))]
    Pap,
    #[serde(rename(deserialize = "2"))]
    Chap,
    #[serde(rename(deserialize = "3"))]
    PapChap,
    #[serde(untagged)]
    Unknown(BoxStr),
}

impl fmt::Display for ApnAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Pap => write!(f, "PAP"),
            Self::Chap => write!(f, "CHAP"),
            Self::PapChap => write!(f, "PAP/CHAP"),
            Self::Unknown(raw) => write!(f, "Unknown ({raw})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum NetworkMode {
    #[serde(rename(deserialize = "0"))]
    Auto,
    #[serde(rename(deserialize = "1"))]
    Lte,
    #[serde(rename(deserialize = "2"))]
    Umts,
    #[serde(untagged)]
    Unknown(BoxStr),
}

impl fmt::Display for NetworkMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "4G/3G"),
            Self::Lte => write!(f, "4G only"),
            Self::Umts => write!(f, "3G only"),
            Self::Unknown(raw) => write!(f, "Unknown ({raw})"),
        }
    }
}

pub fn signal_bars(dbm: i32) -> u8 {
    match dbm {
        d if !(-120..=-44).contains(&d) => 0,
        d if d >= -95 => 4,
        d if d >= -102 => 3,
        d if d >= -108 => 2,
        d if d >= -115 => 1,
        _ => 0,
    }
}

fn signal_label(dbm: i32) -> String {
    format!("{dbm} dBm ({}/4)", signal_bars(dbm))
}

fn print_kv_table(header: &str, rows: &[(&str, String)]) -> EyreResult<()> {
    let mut table = create_table();
    table.set_header([header, "Value"]);
    for (key, value) in rows {
        table.add_row([*key, or_dash(value)]);
    }
    println!("{table}");
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Overview {
    #[serde(rename(deserialize = "SYSVER"))]
    pub firmware_version: BoxStr,
    #[serde(rename(deserialize = "WEBVER"))]
    pub web_version: BoxStr,
    #[serde(rename(deserialize = "HWVER"))]
    pub hardware_version: BoxStr,
    #[serde(rename(deserialize = "SN1"))]
    pub serial_number: BoxStr,
    #[serde(rename(deserialize = "IMEI"))]
    pub imei: BoxStr,
    #[serde(rename(deserialize = "IMSI"))]
    pub imsi: BoxStr,
    #[serde(rename(deserialize = "ICCID"))]
    pub iccid: BoxStr,
    #[serde(rename(deserialize = "SSID"))]
    pub ssid: BoxStr,
    #[serde(rename(deserialize = "WANIP"))]
    pub wan_ip: BoxStr,
    #[serde(rename(deserialize = "SIGNAL"), deserialize_with = "int_or_str")]
    pub signal_dbm: i32,
}

impl HimiGet for Overview {
    const CMDID: &str = "getoverview";
}

impl Show for Overview {
    fn show_json(&self) -> EyreResult<()> {
        print_json(self)
    }

    fn show_table(&self) -> EyreResult<()> {
        print_kv_table(
            "Overview",
            &[
                ("Firmware", self.firmware_version.to_string()),
                ("Web UI", self.web_version.to_string()),
                ("Hardware", self.hardware_version.to_string()),
                ("Serial Number", self.serial_number.to_string()),
                ("IMEI", self.imei.to_string()),
                ("IMSI", self.imsi.to_string()),
                ("ICCID", self.iccid.to_string()),
                ("SSID", self.ssid.to_string()),
                ("WAN IP", self.wan_ip.to_string()),
                ("Signal", signal_label(self.signal_dbm)),
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub carrier: BoxStr,
    #[serde(
        rename(deserialize = "signalStrength"),
        deserialize_with = "int_or_str"
    )]
    pub signal_dbm: i32,
    #[serde(rename(deserialize = "internetState"))]
    pub internet: BoxStr,
    #[serde(rename(deserialize = "simCardState"))]
    pub sim: BoxStr,
    #[serde(rename(deserialize = "wifiapstatus"))]
    pub wifi: BoxStr,
    pub imsi: BoxStr,
}

impl HimiGet for Status {
    const CMDID: &str = "getallstatus";
}

impl Show for Status {
    fn show_json(&self) -> EyreResult<()> {
        print_json(self)
    }

    fn show_table(&self) -> EyreResult<()> {
        print_kv_table(
            "Status",
            &[
                ("Carrier", self.carrier.to_string()),
                ("Signal", signal_label(self.signal_dbm)),
                ("Internet", self.internet.to_string()),
                ("SIM", self.sim.to_string()),
                ("Wi-Fi", self.wifi.to_string()),
                ("IMSI", self.imsi.to_string()),
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemInfo {
    #[serde(rename(deserialize = "systime"))]
    pub system_time: BoxStr,
    #[serde(rename(deserialize = "sysruntime"), deserialize_with = "int_or_str")]
    pub uptime_seconds: u64,
    #[serde(rename(deserialize = "cputemp"), deserialize_with = "int_or_str")]
    pub cpu_temp_celsius: i32,
    #[serde(rename(deserialize = "batcap"), deserialize_with = "int_or_str")]
    pub battery_percent: u8,
    #[serde(rename(deserialize = "wifinum"), deserialize_with = "int_or_str")]
    pub wifi_clients: usize,
    #[serde(rename(deserialize = "ethnum"), deserialize_with = "int_or_str")]
    pub ethernet_clients: usize,
    #[serde(rename(deserialize = "bsid"))]
    pub base_station_id: BoxStr,
    #[serde(rename(deserialize = "cellid"))]
    pub cell_id: BoxStr,
}

impl HimiGet for SystemInfo {
    const CMDID: &str = "getsysinfo";
}

impl Show for SystemInfo {
    fn show_json(&self) -> EyreResult<()> {
        print_json(self)
    }

    fn show_table(&self) -> EyreResult<()> {
        print_kv_table(
            "System",
            &[
                ("System Time", self.system_time.to_string()),
                ("Uptime", Seconds(self.uptime_seconds).to_string()),
                ("CPU Temp", format!("{} °C", self.cpu_temp_celsius)),
                ("Battery", format!("{}%", self.battery_percent)),
                ("Wi-Fi Clients", self.wifi_clients.to_string()),
                ("Ethernet Clients", self.ethernet_clients.to_string()),
                ("Base Station ID", self.base_station_id.to_string()),
                ("Cell ID", self.cell_id.to_string()),
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ConnectedDevices(pub BoxList<ConnectedDevice>);

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectedDevice {
    #[serde(rename(deserialize = "devname"))]
    pub hostname: BoxStr,
    #[serde(rename(deserialize = "devip"))]
    pub ip: BoxStr,
    #[serde(rename(deserialize = "devmac"))]
    pub mac: BoxStr,
    #[serde(rename(deserialize = "devtypestr"))]
    pub connection: BoxStr,
}

impl HimiGet for ConnectedDevices {
    const CMDID: &str = "getconnectedsta";
}

impl Show for ConnectedDevices {
    fn show_json(&self) -> EyreResult<()> {
        print_json(self)
    }

    fn show_table(&self) -> EyreResult<()> {
        if self.0.is_empty() {
            println!("No connected devices.");
            return Ok(());
        }

        let mut table = create_table();
        table.set_header(["Hostname", "IP", "MAC", "Type"]);
        for device in &self.0 {
            table.add_row([
                or_dash(&device.hostname),
                &device.ip,
                &device.mac,
                &device.connection,
            ]);
        }
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WifiInfo {
    #[serde(rename(deserialize = "wifistate"))]
    pub state: WifiState,
    #[serde(rename(deserialize = "wifissid"))]
    pub ssid: BoxStr,
    #[serde(rename(deserialize = "wifipass"))]
    pub password: BoxStr,
    #[serde(rename(deserialize = "wifiauth"))]
    pub security: WifiSecurity,
    #[serde(rename(deserialize = "wifichan"))]
    pub channel: BoxStr,
    #[serde(rename(deserialize = "wifibrocast"))]
    pub broadcast_ssid: bool,
}

impl HimiGet for WifiInfo {
    const CMDID: &str = "getwifiinfo";
}

impl Show for WifiInfo {
    fn show_json(&self) -> EyreResult<()> {
        print_json(self)
    }

    fn show_table(&self) -> EyreResult<()> {
        print_kv_table(
            "Wi-Fi",
            &[
                ("State", self.state.to_string()),
                ("SSID", self.ssid.to_string()),
                ("Password", self.password.to_string()),
                ("Security", self.security.to_string()),
                ("Channel", self.channel.to_string()),
                ("Broadcast SSID", yes_no(self.broadcast_ssid).to_owned()),
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApnInfo {
    #[serde(rename(deserialize = "apnlist"))]
    pub profiles: BoxList<ApnProfile>,
    #[serde(rename(deserialize = "apncurrent"), deserialize_with = "int_or_str")]
    pub active_index: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApnProfile {
    pub id: BoxStr,
    #[serde(rename(deserialize = "apnname"))]
    pub name: BoxStr,
    pub apn: BoxStr,
    #[serde(rename(deserialize = "pdptype"))]
    pub pdp_type: BoxStr,
    pub auth: ApnAuth,
    #[serde(default)]
    pub username: BoxStr,
    #[serde(rename(deserialize = "passw"), default)]
    pub password: BoxStr,
    #[serde(rename(deserialize = "mmc"))]
    pub mcc: BoxStr,
    pub mnc: BoxStr,
    #[serde(default)]
    pub proxy: BoxStr,
    #[serde(default)]
    pub port: BoxStr,
}

impl HimiGet for ApnInfo {
    const CMDID: &str = "getapninfo";
}

impl Show for ApnInfo {
    fn show_json(&self) -> EyreResult<()> {
        print_json(self)
    }

    fn show_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header([
            "Active", "Name", "APN", "PDP Type", "Auth", "Username", "Password", "MCC/MNC",
        ]);
        for (index, profile) in self.profiles.iter().enumerate() {
            table.add_row([
                if index == self.active_index { "*" } else { "" },
                &profile.name,
                &profile.apn,
                &profile.pdp_type,
                &profile.auth.to_string(),
                or_dash(&profile.username),
                or_dash(&profile.password),
                &format!("{}/{}", profile.mcc, profile.mnc),
            ]);
        }
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DhcpInfo {
    #[serde(rename(deserialize = "dhcpserver"))]
    pub enabled: bool,
    #[serde(rename(deserialize = "gwip"))]
    pub gateway: BoxStr,
    pub netmask: BoxStr,
    #[serde(rename(deserialize = "ippoolstart"))]
    pub pool_start: BoxStr,
    #[serde(rename(deserialize = "ippoolend"))]
    pub pool_end: BoxStr,
    #[serde(rename(deserialize = "leasetime"), deserialize_with = "int_or_str")]
    pub lease_hours: u32,
}

impl HimiGet for DhcpInfo {
    const CMDID: &str = "getdhcpinfo";
}

impl Show for DhcpInfo {
    fn show_json(&self) -> EyreResult<()> {
        print_json(self)
    }

    fn show_table(&self) -> EyreResult<()> {
        print_kv_table(
            "DHCP",
            &[
                ("DHCP Server", yes_no(self.enabled).to_owned()),
                ("Gateway", self.gateway.to_string()),
                ("Netmask", self.netmask.to_string()),
                (
                    "IP Pool",
                    format!("{} – {}", self.pool_start, self.pool_end),
                ),
                ("Lease Time", format!("{} h", self.lease_hours)),
            ],
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkModeInfo {
    #[serde(rename(deserialize = "netmode"))]
    pub mode: NetworkMode,
}

impl HimiGet for NetworkModeInfo {
    const CMDID: &str = "getnetworkmode";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimSlot {
    #[serde(rename(deserialize = "simcurrent"), deserialize_with = "int_or_str")]
    pub current: usize,
    #[serde(rename(deserialize = "simset"), deserialize_with = "int_or_str")]
    pub selected: usize,
}

impl HimiGet for SimSlot {
    const CMDID: &str = "getsimset";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsbTether {
    #[serde(rename(deserialize = "usbstatus"), deserialize_with = "flag")]
    pub enabled: bool,
}

impl HimiGet for UsbTether {
    const CMDID: &str = "gethimiusbtether";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionDetect {
    #[serde(rename(deserialize = "state"), deserialize_with = "flag")]
    pub enabled: bool,
    #[serde(rename(deserialize = "ipaddr1"))]
    pub target1: BoxStr,
    #[serde(rename(deserialize = "ipaddr2"))]
    pub target2: BoxStr,
    #[serde(rename(deserialize = "ipaddr3"))]
    pub target3: BoxStr,
}

impl HimiGet for ConnectionDetect {
    const CMDID: &str = "getcondetect";
}

#[derive(Debug, Serialize)]
pub struct NetworkReport {
    pub mode: NetworkModeInfo,
    pub sim: SimSlot,
    pub usb: UsbTether,
    pub detect: ConnectionDetect,
}

impl Show for NetworkReport {
    fn show_json(&self) -> EyreResult<()> {
        #[derive(Serialize)]
        struct NetworkJson<'a> {
            network_mode: &'a NetworkMode,
            sim_in_use: usize,
            sim_selected: usize,
            usb_tether: bool,
            connection_check: &'a ConnectionDetect,
        }

        print_json(&NetworkJson {
            network_mode: &self.mode.mode,
            sim_in_use: self.sim.current + 1,
            sim_selected: self.sim.selected + 1,
            usb_tether: self.usb.enabled,
            connection_check: &self.detect,
        })
    }

    fn show_table(&self) -> EyreResult<()> {
        let targets = [
            self.detect.target1.as_ref(),
            self.detect.target2.as_ref(),
            self.detect.target3.as_ref(),
        ]
        .into_iter()
        .filter(|target| !target.is_empty())
        .collect::<Vec<_>>()
        .join(", ");

        print_kv_table(
            "Network",
            &[
                ("Network Mode", self.mode.mode.to_string()),
                ("SIM In Use", format!("SIM{}", self.sim.current + 1)),
                ("SIM Selected", format!("SIM{}", self.sim.selected + 1)),
                ("USB Tether", yes_no(self.usb.enabled).to_owned()),
                ("Connection Check", yes_no(self.detect.enabled).to_owned()),
                ("Check Targets", targets),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode<T: HimiGet>(json: &str) -> T {
        serde_json::from_str(json).expect("fixture should decode")
    }

    #[test]
    fn overview() {
        let overview: Overview = decode(
            r#"{"SYSVER":"HMIFI_X4S1U1W1_CM_V02_1102","SN1":"250000000000","HWVER":"HW5.0","IMEI":"860000000000000","SSID":"test ssid","IMSI":"636020000000000","WANIP":"10.0.0.2","ICCID":"89251020000000000000","WEBVER":"WEB1.1","SIGNAL":-88}"#,
        );
        assert_eq!(overview.signal_dbm, -88);
        assert_eq!(&*overview.firmware_version, "HMIFI_X4S1U1W1_CM_V02_1102");
    }

    #[test]
    fn status() {
        let status: Status = decode(
            r#"{"signalStrength":-88,"carrier":"Safaricom","internetState":"connected","imsi":"636020000000000","simCardState":"ok","wifiapstatus":"on"}"#,
        );
        assert_eq!(&*status.internet, "connected");
        assert_eq!(&*status.sim, "ok");
        assert_eq!(&*status.wifi, "on");
    }

    #[test]
    fn system_info() {
        let info: SystemInfo = decode(
            r#"{"systime":"2026-10-31 16:19:52","bsid":"14000","cellid":"36367134","cputemp":"58","sysruntime":11899,"wifinum":"3","ethnum":"0","batcap":"88"}"#,
        );
        assert_eq!(info.uptime_seconds, 11899);
        assert_eq!(info.cpu_temp_celsius, 58);
        assert_eq!(info.battery_percent, 88);
        assert_eq!(info.wifi_clients, 3);
    }

    #[test]
    fn connected_devices() {
        let devices: ConnectedDevices = decode(
            r#"[{"devip":"192.168.100.239","devname":"tv","devtype":0,"devmac":"00:00:00:00:00:01","devtypestr":"WIFI"},{"devip":"192.168.100.15","devname":"tablet","devtype":1,"devmac":"00:00:00:00:00:02","devtypestr":"WIFI"}]"#,
        );
        assert_eq!(devices.0.len(), 2);
        assert_eq!(&*devices.0[0].connection, "WIFI");
    }

    #[test]
    fn wifi_info() {
        let wifi: WifiInfo = decode(
            r#"{"wifibrocast":true,"wifipass":"password","wifistate":"open","wifiauth":"4","wifissid":"test ssid","wifichan":"1"}"#,
        );
        assert_eq!(wifi.state, WifiState::On);
        assert_eq!(wifi.security, WifiSecurity::Wpa2Psk);
        assert!(wifi.broadcast_ssid);
    }

    #[test]
    fn apn_info_with_sparse_profiles() {
        let apn: ApnInfo = decode(
            r#"{"apnlist":[{"id":"3857","mmc":"636","mnc":"02","apnname":"safaricom","pdptype":"IP","apn":"safaricom","auth":"2"},{"port":"","id":"4387","passw":"","username":"","apnname":"safaricom et","proxy":"","mmc":"636","mnc":"02","pdptype":"IPV4V6","apn":"safaricom","auth":"0"}],"apncurrent":1}"#,
        );
        assert_eq!(apn.profiles.len(), 2);
        assert_eq!(apn.active_index, 1);
        assert_eq!(apn.profiles[0].auth, ApnAuth::Chap);
        assert!(apn.profiles[0].username.is_empty());
    }

    #[test]
    fn dhcp_info() {
        let dhcp: DhcpInfo = decode(
            r#"{"leasetime":"1","ippoolstart":"192.168.100.2","netmask":"255.255.255.0","gwip":"192.168.100.1","ippoolend":"192.168.100.254","dhcpserver":true}"#,
        );
        assert!(dhcp.enabled);
        assert_eq!(dhcp.lease_hours, 1);
    }

    #[test]
    fn network_pieces() {
        let mode: NetworkModeInfo = decode(r#"{"netmode":"1"}"#);
        let sim: SimSlot = decode(r#"{"spassw":"","simset":0,"simcurrent":0}"#);
        let usb: UsbTether = decode(r#"{"usbstatus":"1"}"#);
        let detect: ConnectionDetect = decode(
            r#"{"ipaddr1":"4.2.2.2","state":"0","ipaddr3":"www.google.com","ipaddr2":"8.8.8.8"}"#,
        );
        assert_eq!(mode.mode, NetworkMode::Lte);
        assert_eq!(sim.current, 0);
        assert!(usb.enabled);
        assert!(!detect.enabled);
    }

    #[test]
    fn unknown_codes_are_kept() {
        let wifi: WifiInfo = decode(
            r#"{"wifibrocast":false,"wifipass":"","wifistate":"close","wifiauth":"7","wifissid":"x","wifichan":"6"}"#,
        );
        assert_eq!(wifi.security, WifiSecurity::Unknown("7".into()));
        assert_eq!(wifi.security.to_string(), "Unknown (7)");
    }

    #[test]
    fn signal_bars_match_web_ui() {
        assert_eq!(signal_bars(-88), 4);
        assert_eq!(signal_bars(-100), 3);
        assert_eq!(signal_bars(-105), 2);
        assert_eq!(signal_bars(-110), 1);
        assert_eq!(signal_bars(-118), 0);
        assert_eq!(signal_bars(-30), 0);
    }
}
