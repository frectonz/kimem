use crate::common::*;
use serde::{Deserialize, Serialize};

pub trait ProcGet: serde::de::DeserializeOwned {
    const CMD: &str;
    type Params: serde::ser::Serialize + Default;

    fn print_table(&self) -> EyreResult<()>;
}

#[derive(Debug, Deserialize)]
pub struct GetRandomLogin {
    pub random_login: BoxStr,
}

impl ProcGet for GetRandomLogin {
    const CMD: &str = "get_random_login";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Random Login"])
            .add_row([&self.random_login]);

        println!("{table}");

        Ok(())
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

    fn print_table(&self) -> EyreResult<()> {
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

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Imei {
    pub imei: BoxStr,
}

impl ProcGet for Imei {
    const CMD: &str = "imei";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["IMEI"]).add_row([&self.imei]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct SimImsi {
    pub sim_imsi: BoxStr,
}

impl ProcGet for SimImsi {
    const CMD: &str = "sim_imsi";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["SIM IMSI"]).add_row([&self.sim_imsi]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct NetworkType {
    pub network_type: BoxStr,
}

impl ProcGet for NetworkType {
    const CMD: &str = "network_type";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table
            .set_header(["Network Type"])
            .add_row([&self.network_type]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct SimPlmn {
    pub sim_plmn: BoxStr,
}

impl ProcGet for SimPlmn {
    const CMD: &str = "sim_plmn";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["SIM PLMN"]).add_row([&self.sim_plmn]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Rssi {
    pub rssi: BoxStr,
}

impl ProcGet for Rssi {
    const CMD: &str = "rssi";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["RSSI (dBm)"]).add_row([&self.rssi]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Rscp {
    pub rscp: BoxStr,
}

impl ProcGet for Rscp {
    const CMD: &str = "rscp";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["RSCP (dBm)"]).add_row([&self.rscp]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct LteRsrq {
    pub lte_rsrq: BoxStr,
}

impl ProcGet for LteRsrq {
    const CMD: &str = "lte_rsrq";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table
            .set_header(["LTE RSRQ (dBm)"])
            .add_row([&self.lte_rsrq]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct LteRsrp {
    pub lte_rsrp: BoxStr,
}

impl ProcGet for LteRsrp {
    const CMD: &str = "lte_rsrp";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["LTE RSRP (dBm)"])
            .add_row([&self.lte_rsrp]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct LteCellId {
    pub lte_cellid: BoxStr,
}

impl ProcGet for LteCellId {
    const CMD: &str = "lte_cellid";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["LTE Cell ID"])
            .add_row([&self.lte_cellid]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct LteEnodebId {
    pub lte_enodebid: BoxStr,
}

impl ProcGet for LteEnodebId {
    const CMD: &str = "lte_enodebid";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["LTE eNodeB ID"])
            .add_row([&self.lte_enodebid]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct LteTac {
    pub lte_tac: BoxStr,
}

impl ProcGet for LteTac {
    const CMD: &str = "lte_tac";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["LTE TAC"]).add_row([&self.lte_tac]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct SignalBar {
    pub signalbar: BoxStr,
}

impl ProcGet for SignalBar {
    const CMD: &str = "signalbar";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Signal Bar"]).add_row([&self.signalbar]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct AirtimeBalance {
    pub airtime_balance: BoxStr,
}

impl ProcGet for AirtimeBalance {
    const CMD: &str = "airtime_balance";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        let balance = ucs2_decode(&self.airtime_balance)?;
        table.set_header(["Airtime Balance"]).add_row([&balance]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct WanIpaddr {
    pub wan_ipaddr: BoxStr,
}

impl ProcGet for WanIpaddr {
    const CMD: &str = "wan_ipaddr";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["WAN IP"]).add_row([&self.wan_ipaddr]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct PppStatus {
    pub ppp_status: BoxStr,
}

impl ProcGet for PppStatus {
    const CMD: &str = "ppp_status";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["PPP Status"]).add_row([&self.ppp_status]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct LanIpaddr {
    pub lan_ipaddr: BoxStr,
}

impl ProcGet for LanIpaddr {
    const CMD: &str = "lan_ipaddr";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["LAN IP"]).add_row([&self.lan_ipaddr]);
        println!("{table}");

        Ok(())
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

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Local Domain"])
            .add_row([&self.local_domain]);

        println!("{table}");

        Ok(())
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

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["DHCP Start"]).add_row([&self.dhcp_start]);
        println!("{table}");

        Ok(())
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

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["DHCP End"]).add_row([&self.dhcp_end]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct BatteryPercentage {
    pub battery_percentage: BoxStr,
}

impl ProcGet for BatteryPercentage {
    const CMD: &str = "battery_percentage";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Battery Percentage"])
            .add_row([&self.battery_percentage]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct BatteryExist {
    pub battery_exist: BoxStr,
}

impl ProcGet for BatteryExist {
    const CMD: &str = "battery_exist";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Battery Exists"])
            .add_row([&self.battery_exist]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct PowerExist {
    pub power_exist: BoxStr,
}

impl ProcGet for PowerExist {
    const CMD: &str = "power_exist";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Power Exists"])
            .add_row([&self.power_exist]);

        println!("{table}");

        Ok(())
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
        match self {
            MessageTagStatus::Read => write!(f, "READ"),
            MessageTagStatus::Unread => write!(f, "UNREAD"),
            MessageTagStatus::Sent => write!(f, "SENT"),
            MessageTagStatus::Unknown { tag } => write!(f, "UNKNOW({tag})"),
        }
    }
}

impl ProcGet for SmsInbox {
    const CMD: &str = "sms_data_total";
    type Params = SmsInboxParams;

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table.set_header(["ID", "Number", "Content", "Status", "Date"]);

        for d in self.messages.iter() {
            let mut content = ucs2_decode(&d.content)?.trim().to_owned();
            if content.len() > 24 {
                content.truncate(24);
                content = content.trim_end().to_owned()
            };

            let tag = MessageTagStatus::from_tag(&d.tag).to_string();
            let datetime = Datetime::parse(&d.date)?.to_string();

            table.add_row([
                d.id.as_ref(),
                d.number.as_ref(),
                content.as_str(),
                tag.as_str(),
                datetime.as_str(),
            ]);
        }

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct NvArfcn {
    pub nv_arfcn: BoxStr,
}

impl ProcGet for NvArfcn {
    const CMD: &str = "nv_arfcn";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["NV ARFCN"]).add_row([&self.nv_arfcn]);
        println!("{table}");

        Ok(())
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

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Primary SSID"]).add_row([&self.ssid1]);
        println!("{table}");

        Ok(())
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

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Secondary SSID"]).add_row([&self.m_ssid]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct PrimarySsidPsk {
    #[serde(rename = "WPAPSK1_encode")]
    pub wpapsk1_encode: BoxStr,
}

impl ProcGet for PrimarySsidPsk {
    const CMD: &str = "WPAPSK1_encode";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        let psk = b64_decode(&self.wpapsk1_encode)?;
        table.set_header(["Primary SSID Password"]).add_row([psk]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct SecondarySsidPsk {
    #[serde(rename = "m_WPAPSK1_encode")]
    pub m_wpapsk1_encode: BoxStr,
}

impl ProcGet for SecondarySsidPsk {
    const CMD: &str = "m_WPAPSK1_encode";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        let psk = b64_decode(&self.m_wpapsk1_encode)?;
        table.set_header(["Secondary SSID Password"]).add_row([psk]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct PrimarySsidAuthMode {
    #[serde(rename = "AuthMode")]
    pub auth_mode: BoxStr,
}

impl ProcGet for PrimarySsidAuthMode {
    const CMD: &str = "AuthMode";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Primary SSID Auth Mode"])
            .add_row([&self.auth_mode]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct SecondarySsidAuthMode {
    #[serde(rename = "m_AuthMode")]
    pub m_auth_mode: BoxStr,
}

impl ProcGet for SecondarySsidAuthMode {
    const CMD: &str = "m_AuthMode";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Secondary SSID Auth Mode"])
            .add_row([&self.m_auth_mode]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct MonthlyRx {
    pub monthly_rx_bytes: BoxStr,
}

impl ProcGet for MonthlyRx {
    const CMD: &str = "monthly_rx_bytes";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        let size = self.monthly_rx_bytes.parse::<usize>()?;
        let size = humansize::format_size(size, humansize::DECIMAL);

        table.set_header(["Monthly RX"]).add_row([&size]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct MonthlyTx {
    pub monthly_tx_bytes: BoxStr,
}

impl ProcGet for MonthlyTx {
    const CMD: &str = "monthly_tx_bytes";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        let size = self.monthly_tx_bytes.parse::<usize>()?;
        let size = humansize::format_size(size, humansize::DECIMAL);

        table.set_header(["Monthly TX"]).add_row([&size]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct RealtimeRx {
    pub realtime_rx_bytes: BoxStr,
}

impl ProcGet for RealtimeRx {
    const CMD: &str = "realtime_rx_bytes";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        let size = self.realtime_rx_bytes.parse::<usize>()?;
        let size = humansize::format_size(size, humansize::DECIMAL);

        table.set_header(["Realtime RX"]).add_row([&size]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct RealtimeTx {
    pub realtime_tx_bytes: BoxStr,
}

impl ProcGet for RealtimeTx {
    const CMD: &str = "realtime_tx_bytes";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        let size = self.realtime_tx_bytes.parse::<usize>()?;
        let size = humansize::format_size(size, humansize::DECIMAL);

        table.set_header(["Realtime TX"]).add_row([&size]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct CrVersion {
    pub cr_version: BoxStr,
}

impl ProcGet for CrVersion {
    const CMD: &str = "cr_version";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["CR Version"]).add_row([&self.cr_version]);
        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct HardwareVersion {
    pub hardware_version: BoxStr,
}

impl ProcGet for HardwareVersion {
    const CMD: &str = "hardware_version";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Hardware Version"])
            .add_row([&self.hardware_version]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct SystemStatus {
    pub platform_version: BoxStr,
    pub sim_status: BoxStr,
    pub network_type: BoxStr,
    pub service_status: BoxStr,
    pub plmn: BoxStr,
    pub plmn_id: BoxStr,
    pub cell_id: BoxStr,
    pub enode_id: BoxStr,
    pub w_cellid: BoxStr,
    pub w_rncid: BoxStr,
    pub gsm_cellid: BoxStr,
    #[serde(rename = "dwCellId")]
    pub dw_cell_id: BoxStr,
    pub phy_cell_id: BoxStr,
    pub band: BoxStr,
    pub rsrp: BoxStr,
    pub rsrq: BoxStr,
    pub sinr: BoxStr,
    pub roam: BoxStr,
    pub tz_lock_current_cellid: BoxStr,
    pub mcs: BoxStr,
    pub cqi: BoxStr,
    pub rssi: BoxStr,
    pub wan_ip: BoxStr,
    pub lte_band: BoxStr,
    pub uplink_rate: BoxStr,
    pub downlink_rate: BoxStr,
    pub uplink_traffic: BoxStr,
    pub downlink_traffic: BoxStr,
    pub limit_switch: BoxStr,
    pub main_antenna: BoxStr,
    pub sub_antenna: BoxStr,
    pub only_antenna: BoxStr,
    pub dns: BoxStr,
    pub online_time: BoxStr,
    pub networking_duration: BoxStr,
    pub wifi_channel: BoxStr,
    pub gateway: BoxStr,
    pub netmask: BoxStr,
}

impl ProcGet for SystemStatus {
    const CMD: &str = "system_status";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["System Config", "Value"]);

        table.add_row(["Platform Version", &self.platform_version]);
        table.add_row(["SIM Status", &self.sim_status]);
        table.add_row(["Network Type", &self.network_type]);
        table.add_row(["Service Status", &self.service_status]);
        table.add_row(["PLMN", &self.plmn]);
        table.add_row(["PLMN ID", &self.plmn_id]);
        table.add_row(["Cell ID", &self.cell_id]);
        table.add_row(["eNode ID", &self.enode_id]);
        table.add_row(["W Cell ID", &self.w_cellid]);
        table.add_row(["W RNC ID", &self.w_rncid]);
        table.add_row(["GSM Cell ID", &self.gsm_cellid]);
        table.add_row(["DW Cell ID", &self.dw_cell_id]);
        table.add_row(["PHY Cell ID", &self.phy_cell_id]);
        table.add_row(["Band", &self.band]);
        table.add_row(["RSRP", &self.rsrp]);
        table.add_row(["RSRQ", &self.rsrq]);
        table.add_row(["SINR", &self.sinr]);
        table.add_row(["Roam", &self.roam]);
        table.add_row(["TZ Lock Currect Cell ID", &self.tz_lock_current_cellid]);
        table.add_row(["MCS", &self.mcs]);
        table.add_row(["CQI", &self.cqi]);
        table.add_row(["RSSI", &self.rssi]);
        table.add_row(["WAN IP", &self.wan_ip]);
        table.add_row(["LTE Band", &self.lte_band]);
        table.add_row(["Uplink Rate", &self.uplink_rate]);
        table.add_row(["Downlink Rate", &self.downlink_rate]);
        table.add_row(["Limit Switch", &self.limit_switch]);
        table.add_row(["Main Antenna", &self.main_antenna]);
        table.add_row(["Sub Antenna", &self.sub_antenna]);
        table.add_row(["Only Antenna", &self.only_antenna]);
        table.add_row(["DNS", &self.dns]);
        table.add_row(["Online Time", &self.online_time]);
        table.add_row(["Networking Duration", &self.networking_duration]);
        table.add_row(["WIFI Channel", &self.wifi_channel]);
        table.add_row(["Gateway", &self.gateway]);
        table.add_row(["NetMask", &self.netmask]);

        println!("{table}");
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct MaxStationNum {
    #[serde(rename = "MAX_Station_num")]
    pub max_station_num: BoxStr,
}

impl ProcGet for MaxStationNum {
    const CMD: &str = "MAX_Station_num";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Max Station Num"])
            .add_row([&self.max_station_num]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceVersion {
    pub device_version: BoxStr,
}

impl ProcGet for DeviceVersion {
    const CMD: &str = "device_version";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Device Version"])
            .add_row([&self.device_version]);

        println!("{table}");

        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub struct SmsParameterInfo {
    pub sms_para_sca: BoxStr,
    pub sms_para_mem_store: BoxStr,
    pub sms_para_validity_period: BoxStr,
    pub sms_para_status_report: BoxStr,
    pub sendfail_retry: BoxStr,
    pub outdate_delete: BoxStr,
    pub default_store: BoxStr,
}

impl ProcGet for SmsParameterInfo {
    const CMD: &str = "sms_parameter_info";
    type Params = ();

    fn print_table(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["SMS Parameter", "Value"])
            .add_row(["Service Center Address", &self.sms_para_sca])
            .add_row(["Memory store", &self.sms_para_mem_store])
            .add_row(["Validity period", &self.sms_para_validity_period])
            .add_row(["Status report", &self.sms_para_status_report])
            .add_row(["Should retry", &self.sendfail_retry])
            .add_row(["Should delete outdated", &self.outdate_delete])
            .add_row(["Default storage location", &self.default_store]);

        println!("{table}");

        Ok(())
    }
}
