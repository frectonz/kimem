use crate::common::*;
use serde::Deserialize;

pub trait ProcGet {
    const CMD: &str;
    type Params: serde::ser::Serialize + Default;
    type Response: serde::de::DeserializeOwned;

    fn print_table(&self);
}

#[derive(Debug, Deserialize)]
pub struct GetRandomLogin {
    pub random_login: BoxStr,
}

impl ProcGet for GetRandomLogin {
    const CMD: &str = "get_random_login";
    type Params = ();
    type Response = GetRandomLogin;

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
    type Response = StationList;

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
    type Response = Imei;

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
    type Response = SimImsi;

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
    type Response = NetworkType;

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
    type Response = SimPlmn;

    fn print_table(&self) {
        let mut table = create_table();
        table.set_header(["SIM PLMN"]).add_row([&self.sim_plmn]);
        println!("{table}");
    }
}
