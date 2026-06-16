use crate::common::*;
use serde::Deserialize;

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
