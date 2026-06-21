use crate::common::*;
use serde::{Deserialize, Serialize};

pub trait ProcGet: serde::de::DeserializeOwned {
    const CMD: &str;
    type Params: serde::ser::Serialize + Default;

    fn print_table(&self);
}

#[derive(Debug, Deserialize)]
pub struct GetRandomLogin {
    pub random_login: BoxStr,
}

impl ProcGet for GetRandomLogin {
    const CMD: &str = "get_random_login";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();

        table
            .set_header(["Random Login"])
            .add_row([&self.random_login]);

        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct StationList {
    pub station_list: Vec<ConnectedDevice>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectedDevice {
    pub connect_time: BoxStr,
    pub ssid_index: BoxStr,
    pub dev_type: BoxStr,
    pub mac_addr: BoxStr,
    pub hostname: BoxStr,
    pub ip_addr: BoxStr,
    pub ipv6: BoxStr,
    pub ipv6_local: BoxStr,
    pub ip_type: BoxStr,
}

impl ProcGet for StationList {
    const CMD: &str = "station_list";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();

        table.set_header([
            "Dev Type",
            "MAC Address",
            "Hostname",
            "IP Address",
            "IP Type",
        ]);

        for d in self.station_list.iter() {
            table.add_row([
                &d.dev_type,
                &d.mac_addr,
                &d.hostname,
                &d.ip_addr,
                &d.ip_type,
            ]);
        }

        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct Imei {
    pub imei: BoxStr,
}

impl ProcGet for Imei {
    const CMD: &str = "imei";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["IMEI"]).add_row([&self.imei]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct SimImsi {
    pub sim_imsi: BoxStr,
}

impl ProcGet for SimImsi {
    const CMD: &str = "sim_imsi";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["SIM IMSI"]).add_row([&self.sim_imsi]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct NetworkType {
    pub network_type: BoxStr,
}

impl ProcGet for NetworkType {
    const CMD: &str = "network_type";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table
            .set_header(["Network Type"])
            .add_row([&self.network_type]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct SimPlmn {
    pub sim_plmn: BoxStr,
}

impl ProcGet for SimPlmn {
    const CMD: &str = "sim_plmn";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["SIM PLMN"]).add_row([&self.sim_plmn]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct Rssi {
    pub rssi: BoxStr,
}

impl ProcGet for Rssi {
    const CMD: &str = "rssi";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["RSSI (dBm)"]).add_row([&self.rssi]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct Rscp {
    pub rscp: BoxStr,
}

impl ProcGet for Rscp {
    const CMD: &str = "rscp";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["RSCP (dBm)"]).add_row([&self.rscp]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct LteRsrq {
    pub lte_rsrq: BoxStr,
}

impl ProcGet for LteRsrq {
    const CMD: &str = "lte_rsrq";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table
            .set_header(["LTE RSRQ (dBm)"])
            .add_row([&self.lte_rsrq]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct LteRsrp {
    pub lte_rsrp: BoxStr,
}

impl ProcGet for LteRsrp {
    const CMD: &str = "lte_rsrp";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table
            .set_header(["LTE RSRP (dBm)"])
            .add_row([&self.lte_rsrp]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct LteCellId {
    pub lte_cellid: BoxStr,
}

impl ProcGet for LteCellId {
    const CMD: &str = "lte_cellid";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table
            .set_header(["LTE Cell ID"])
            .add_row([&self.lte_cellid]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct LteEnodebId {
    pub lte_enodebid: BoxStr,
}

impl ProcGet for LteEnodebId {
    const CMD: &str = "lte_enodebid";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table
            .set_header(["LTE eNodeB ID"])
            .add_row([&self.lte_enodebid]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct LteTac {
    pub lte_tac: BoxStr,
}

impl ProcGet for LteTac {
    const CMD: &str = "lte_tac";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["LTE TAC"]).add_row([&self.lte_tac]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct SignalBar {
    pub signalbar: BoxStr,
}

impl ProcGet for SignalBar {
    const CMD: &str = "signalbar";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["Signal Bar"]).add_row([&self.signalbar]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct AirtimeBalance {
    pub airtime_balance: BoxStr,
}

impl ProcGet for AirtimeBalance {
    const CMD: &str = "airtime_balance";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        let balance = decode_ucs2_be(&self.airtime_balance).unwrap_or_default();
        table.set_header(["Airtime Balance"]).add_row([&balance]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct WanIpaddr {
    pub wan_ipaddr: BoxStr,
}

impl ProcGet for WanIpaddr {
    const CMD: &str = "wan_ipaddr";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["WAN IP"]).add_row([&self.wan_ipaddr]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct PppStatus {
    pub ppp_status: BoxStr,
}

impl ProcGet for PppStatus {
    const CMD: &str = "ppp_status";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["PPP Status"]).add_row([&self.ppp_status]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct LanIpaddr {
    pub lan_ipaddr: BoxStr,
}

impl ProcGet for LanIpaddr {
    const CMD: &str = "lan_ipaddr";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["LAN IP"]).add_row([&self.lan_ipaddr]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct LocalDomain {
    #[serde(rename = "LocalDomain")]
    pub local_domain: BoxStr,
}

impl ProcGet for LocalDomain {
    const CMD: &str = "LocalDomain";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table
            .set_header(["Local Domain"])
            .add_row([&self.local_domain]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct DhcpStart {
    #[serde(rename = "dhcpStart")]
    pub dhcp_start: BoxStr,
}

impl ProcGet for DhcpStart {
    const CMD: &str = "dhcpStart";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["DHCP Start"]).add_row([&self.dhcp_start]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct DhcpEnd {
    #[serde(rename = "dhcpEnd")]
    pub dhcp_end: BoxStr,
}

impl ProcGet for DhcpEnd {
    const CMD: &str = "dhcpEnd";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["DHCP End"]).add_row([&self.dhcp_end]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct CrVersion {
    pub cr_version: BoxStr,
}

impl ProcGet for CrVersion {
    const CMD: &str = "cr_version";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["CR Version"]).add_row([&self.cr_version]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct BatteryPercentage {
    pub battery_percentage: BoxStr,
}

impl ProcGet for BatteryPercentage {
    const CMD: &str = "battery_percentage";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table
            .set_header(["Battery Percentage"])
            .add_row([&self.battery_percentage]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct BatteryExist {
    pub battery_exist: BoxStr,
}

impl ProcGet for BatteryExist {
    const CMD: &str = "battery_exist";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table
            .set_header(["Battery Exists"])
            .add_row([&self.battery_exist]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct PowerExist {
    pub power_exist: BoxStr,
}

impl ProcGet for PowerExist {
    const CMD: &str = "power_exist";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table
            .set_header(["Power Exists"])
            .add_row([&self.power_exist]);
        println!("{table}");
    }
}

#[derive(Debug, Serialize)]
pub struct SmsInboxParams {
    page: BoxStr,
    data_per_page: BoxStr,
    mem_store: BoxStr,
    tags: BoxStr,
    order_by: BoxStr,
}

impl Default for SmsInboxParams {
    fn default() -> Self {
        Self {
            page: "0".into(),
            data_per_page: "500".into(),
            mem_store: "1".into(),
            tags: "10".into(),
            order_by: "order by id asc".into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SmsInbox {
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: BoxStr,
    pub number: BoxStr,
    pub content: BoxStr,
    pub tag: BoxStr,
    pub date: BoxStr,
    pub draft_group_id: BoxStr,
}

enum MessageTagStatus {
    Read,
    Unread,
    Sent,
    Unknown { tag: BoxStr },
}

impl MessageTagStatus {
    fn from_tag(tag: &str) -> Self {
        match tag {
            "0" => Self::Read,
            "1" => Self::Unread,
            "2" => Self::Sent,
            _ => Self::Unknown { tag: tag.into() },
        }
    }
}

impl std::fmt::Display for MessageTagStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MessageTagStatus::Read => "READ".to_owned(),
            MessageTagStatus::Unread => "UNREAD".to_owned(),
            MessageTagStatus::Sent => "SENT".to_owned(),
            MessageTagStatus::Unknown { tag } => format!("UNKNOW({tag})"),
        };

        write!(f, "{s}")
    }
}

impl ProcGet for SmsInbox {
    const CMD: &str = "sms_data_total";
    type Params = SmsInboxParams;

    fn print_table(&self) {
        let mut table = create_table();

        table.set_header(["ID", "Number", "Content", "Status", "Date"]);

        for d in self.messages.iter() {
            let mut content = decode_ucs2_be(&d.content)
                .unwrap_or_default()
                .trim()
                .to_owned();

            if content.len() > 24 {
                content.truncate(24);
                content.push_str("...");
            }

            let tag = MessageTagStatus::from_tag(&d.tag).to_string();

            let datetime = parse_datetime(&d.date)
                .unwrap()
                .strftime("%F %r")
                .to_string();

            let row: [&str; 5] = [&d.id, &d.number, &content, &tag, &datetime];

            table.add_row(row);
        }

        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct NvArfcn {
    pub nv_arfcn: BoxStr,
}

impl ProcGet for NvArfcn {
    const CMD: &str = "nv_arfcn";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["NV ARFCN"]).add_row([&self.nv_arfcn]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct PrimarySsid {
    #[serde(rename = "SSID1")]
    pub ssid1: BoxStr,
}

impl ProcGet for PrimarySsid {
    const CMD: &str = "SSID1";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["Primary SSID"]).add_row([&self.ssid1]);
        println!("{table}");
    }
}

#[derive(Debug, Deserialize)]
pub struct SecondarySsid {
    #[serde(rename = "m_SSID")]
    pub m_ssid: BoxStr,
}

impl ProcGet for SecondarySsid {
    const CMD: &str = "m_SSID";
    type Params = ();

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["Secondary SSID"]).add_row([&self.m_ssid]);
        println!("{table}");
    }
}
