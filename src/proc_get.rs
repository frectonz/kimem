use crate::common::*;
use crate::multi_get::ProcGetMulti;
use crate::types::*;
use serde::{Deserialize, Serialize};

/// A read from `/reqproc/proc_get` backed by a single `cmd`.
pub trait ProcGet: serde::de::DeserializeOwned {
    const CMD: &str;
    type Params: serde::ser::Serialize + Default + Send + Sync;
}

#[derive(Debug, Deserialize)]
pub struct GetRandomLogin {
    pub random_login: BoxStr,
}

impl ProcGet for GetRandomLogin {
    const CMD: &str = "get_random_login";
    type Params = ();
}

#[derive(Debug, Deserialize)]
pub struct StationList {
    pub station_list: BoxList<ConnectedDevice>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectedDevice {
    pub hostname: BoxStr,
    pub ip_addr: BoxStr,
    pub mac_addr: BoxStr,
}

impl ProcGet for StationList {
    const CMD: &str = "station_list";
    type Params = ();
}

impl Show for StationList {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Hostname", "IP Address", "MAC Address"]);

        for device in &self.station_list {
            table.add_row([or_dash(&device.hostname), &device.ip_addr, &device.mac_addr]);
        }

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
    pub messages: BoxList<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(deserialize_with = "de::from_str")]
    pub id: usize,
    pub number: BoxStr,
    #[serde(deserialize_with = "de::ucs2")]
    pub content: BoxStr,
    pub tag: MessageStatus,
    pub date: Datetime,
}

impl ProcGet for SmsInbox {
    const CMD: &str = "sms_data_total";
    type Params = SmsInboxParams;
}

impl Show for SmsInbox {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["ID", "Number", "Content", "Status", "Date"]);

        for message in &self.messages {
            let id = message.id.to_string();
            let content = truncate_chars(message.content.trim(), 24);
            let tag = message.tag.to_string();
            let date = message.date.to_string();

            table.add_row([id.as_str(), &message.number, &content, &tag, &date]);
        }

        println!("{table}");

        Ok(())
    }
}

impl Show for Message {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Message", "Value"])
            .add_row(["ID", &self.id.to_string()])
            .add_row(["Number", &self.number])
            .add_row(["Status", &self.tag.to_string()])
            .add_row(["Date", &self.date.to_string()])
            .add_row(["Content", self.content.trim()]);

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct AirtimeBalance {
    /// Cached USSD text like "Airtime balance Br. 2247.50. SMS will be
    /// sent.\n1. 60 Min @20 birr\n...".
    #[serde(deserialize_with = "de::ucs2")]
    pub airtime_balance: BoxStr,
}

impl AirtimeBalance {
    /// Pull the money amount ("Br. 2,247.50") out of the USSD text.
    fn balance(&self) -> Option<BoxStr> {
        let after = self.airtime_balance.split_once("Br.")?.1;

        let amount: BoxStr = after
            .trim_start()
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == ',' || *c == '.')
            .collect();
        let amount = amount.trim_end_matches('.');

        (!amount.is_empty()).then(|| format!("Br. {amount}").into_boxed_str())
    }
}

impl ProcGet for AirtimeBalance {
    const CMD: &str = "airtime_balance";
    type Params = ();
}

impl Show for AirtimeBalance {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();
        table.set_header(["Airtime Balance"]);

        // Fall back to the full USSD text when the balance isn't where
        // we expect it (different language, different menu, ...).
        match self.balance() {
            Some(balance) => table.add_row([balance.as_ref()]),
            None => table.add_row([or_dash(self.airtime_balance.trim())]),
        };

        println!("{table}");

        Ok(())
    }
}

/// Signal metrics live in the `system_status` read; the standalone `rssi`
/// cmd echoes RSRP instead of the real RSSI on this firmware. The cell
/// identifiers are kept as text since they're empty outside LTE.
#[derive(Debug, Deserialize)]
pub struct Signal {
    pub rsrp: Dbm,
    pub rsrq: Decibels,
    pub rssi: Dbm,
    pub sinr: Decibels,
    pub band: BoxStr,
    pub cell_id: BoxStr,
    pub enode_id: BoxStr,
    #[serde(rename = "dwCellId")]
    pub full_cell_id: BoxStr,
    pub phy_cell_id: BoxStr,
    pub mcs: BoxStr,
    pub cqi: BoxStr,
}

impl ProcGet for Signal {
    const CMD: &str = "system_status";
    type Params = ();
}

/// TAC and EARFCN aren't part of `system_status`; `get signal` joins this
/// batch onto the [`Signal`] read.
#[derive(Debug, Deserialize)]
pub struct CellExtras {
    pub lte_tac: BoxStr,
    pub nv_arfcn: BoxStr,
}

impl ProcGetMulti for CellExtras {
    const CMDS: &[&str] = &["lte_tac", "nv_arfcn"];
}

#[derive(Debug)]
pub struct SignalReport {
    pub signal: Signal,
    pub cell: CellExtras,
}

impl Show for SignalReport {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Signal", "Value"])
            .add_row(["RSRP", &self.signal.rsrp.to_string()])
            .add_row(["RSRQ", &self.signal.rsrq.to_string()])
            .add_row(["RSSI", &self.signal.rssi.to_string()])
            .add_row(["SINR", &self.signal.sinr.to_string()])
            .add_row(["Band", or_dash(&self.signal.band)])
            .add_row(["EARFCN", or_dash(&self.cell.nv_arfcn)])
            .add_row(["TAC", or_dash(&self.cell.lte_tac)])
            .add_row(["Cell ID", or_dash(&self.signal.cell_id)])
            .add_row(["eNodeB ID", or_dash(&self.signal.enode_id)])
            .add_row(["Full Cell ID", or_dash(&self.signal.full_cell_id)])
            .add_row(["PCI", or_dash(&self.signal.phy_cell_id)])
            .add_row(["MCS", or_dash(&self.signal.mcs)])
            .add_row(["CQI", or_dash(&self.signal.cqi)]);

        println!("{table}");

        Ok(())
    }
}

/// Device identity comes from the `home_get` read; the serial number and
/// uptime aren't available as standalone cmds on this firmware.
#[derive(Debug, Deserialize)]
pub struct Device {
    pub device_version: BoxStr,
    pub sn: BoxStr,
    pub imei: BoxStr,
    pub online_time: Seconds,
}

impl ProcGet for Device {
    const CMD: &str = "home_get";
    type Params = ();
}

/// The SIM ICCID; `sim_iccid` is empty on this firmware, `ziccid` works.
#[derive(Debug, Deserialize)]
pub struct SimIccid {
    pub ziccid: BoxStr,
}

impl ProcGet for SimIccid {
    const CMD: &str = "ziccid";
    type Params = ();
}

#[derive(Debug)]
pub struct DeviceReport {
    pub device: Device,
    pub sim: SimIccid,
}

impl Show for DeviceReport {
    fn show(&self) -> EyreResult<()> {
        let mut table = create_table();

        table
            .set_header(["Device", "Value"])
            .add_row(["Firmware Version", &self.device.device_version])
            .add_row(["S/N", &self.device.sn])
            .add_row(["IMEI", &self.device.imei])
            .add_row(["SIM ICCID", or_dash(&self.sim.ziccid)])
            .add_row(["Uptime", &self.device.online_time.to_string()]);

        println!("{table}");

        Ok(())
    }
}
