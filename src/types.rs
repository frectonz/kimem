//! Typed values decoded from the router's stringly-typed JSON at the serde
//! level, so the rest of the code works with real Rust types.

use crate::common::*;
use serde::{Deserialize, Deserializer, de::Error as _};
use std::net::IpAddr;

/// `deserialize_with` helpers for fields that don't warrant a dedicated type.
pub mod de {
    use crate::common::*;
    use serde::{Deserialize, Deserializer, de::Error as _};

    /// Parse any `FromStr` value transmitted as a JSON string ("42" → 42).
    pub fn from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let raw = BoxStr::deserialize(deserializer)?;
        raw.trim()
            .parse()
            .map_err(|e| D::Error::custom(format!("cannot parse {raw:?}: {e}")))
    }

    /// Decode a "1" / "0" flag into a bool.
    pub fn flag<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = BoxStr::deserialize(deserializer)?;
        match raw.as_ref() {
            "1" => Ok(true),
            "0" => Ok(false),
            other => Err(D::Error::custom(format!(
                "expected \"0\" or \"1\", got {other:?}"
            ))),
        }
    }

    /// Decode UCS-2 hex text ("00480069" → "Hi").
    pub fn ucs2<'de, D>(deserializer: D) -> Result<BoxStr, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = BoxStr::deserialize(deserializer)?;
        ucs2_decode(&raw).map_err(|e| D::Error::custom(format!("invalid UCS-2 {raw:?}: {e}")))
    }

    /// Decode base64 text (`"NENDRThDNTc="` → `"4CCE8C57"`).
    pub fn b64<'de, D>(deserializer: D) -> Result<BoxStr, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = BoxStr::deserialize(deserializer)?;
        b64_decode(&raw).map_err(|e| D::Error::custom(format!("invalid base64 {raw:?}: {e}")))
    }
}

/// Parse a float the router may leave empty when the metric doesn't apply
/// to the current network type (e.g. SINR outside LTE).
fn optional_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = BoxStr::deserialize(deserializer)?;
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(None);
    }
    raw.parse()
        .map(Some)
        .map_err(|e| D::Error::custom(format!("cannot parse {raw:?}: {e}")))
}

/// A byte counter transmitted as a decimal string ("1237026476").
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Bytes(#[serde(deserialize_with = "de::from_str")] pub u64);

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", humansize::format_size(self.0, humansize::DECIMAL))
    }
}

/// A live throughput in bytes per second ("2339").
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct BytesPerSecond(#[serde(deserialize_with = "de::from_str")] pub u64);

impl std::fmt::Display for BytesPerSecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/s",
            humansize::format_size(self.0, humansize::DECIMAL)
        )
    }
}

/// A memory quantity like "24404 kB", as reported by /proc/meminfo.
#[derive(Debug, Clone, Copy)]
pub struct KibiBytes(pub u64);

impl<'de> Deserialize<'de> for KibiBytes {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = BoxStr::deserialize(deserializer)?;
        raw.trim()
            .trim_end_matches("kB")
            .trim()
            .parse()
            .map(Self)
            .map_err(|e| D::Error::custom(format!("cannot parse {raw:?}: {e}")))
    }
}

impl std::fmt::Display for KibiBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            humansize::format_size(self.0 * 1024, humansize::BINARY)
        )
    }
}

/// A megabyte quantity transmitted as a bare decimal string ("36.15").
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct MegaBytes(#[serde(deserialize_with = "de::from_str")] pub f64);

impl std::fmt::Display for MegaBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} MB", self.0)
    }
}

/// A percentage like "24.78%".
#[derive(Debug, Clone, Copy)]
pub struct Percent(pub f64);

impl<'de> Deserialize<'de> for Percent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = BoxStr::deserialize(deserializer)?;
        raw.trim()
            .trim_end_matches('%')
            .parse()
            .map(Self)
            .map_err(|e| D::Error::custom(format!("cannot parse {raw:?}: {e}")))
    }
}

impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}%", self.0)
    }
}

/// A duration in seconds ("10476" → "2h 54m 36s").
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Seconds(#[serde(deserialize_with = "de::from_str")] pub u64);

impl std::fmt::Display for Seconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (days, rest) = (self.0 / 86_400, self.0 % 86_400);
        let (hours, rest) = (rest / 3_600, rest % 3_600);
        let (minutes, seconds) = (rest / 60, rest % 60);

        let mut parts = Vec::new();
        if days > 0 {
            parts.push(format!("{days}d"));
        }
        if hours > 0 {
            parts.push(format!("{hours}h"));
        }
        if minutes > 0 {
            parts.push(format!("{minutes}m"));
        }
        if seconds > 0 || parts.is_empty() {
            parts.push(format!("{seconds}s"));
        }

        write!(f, "{}", parts.join(" "))
    }
}

/// A signal level in dBm; empty when the metric doesn't apply.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Dbm(#[serde(deserialize_with = "optional_f64")] pub Option<f64>);

impl std::fmt::Display for Dbm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{value} dBm"),
            None => write!(f, "—"),
        }
    }
}

/// A signal ratio in dB; empty when the metric doesn't apply.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Decibels(#[serde(deserialize_with = "optional_f64")] pub Option<f64>);

impl std::fmt::Display for Decibels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{value} dB"),
            None => write!(f, "—"),
        }
    }
}

/// A comma-separated resolver list ("10.44.137.4,10.43.137.4").
#[derive(Debug, Clone)]
pub struct DnsServers(pub BoxList<IpAddr>);

impl<'de> Deserialize<'de> for DnsServers {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = BoxStr::deserialize(deserializer)?;
        raw.split(',')
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
            .map(|entry| {
                entry
                    .parse()
                    .map_err(|e| D::Error::custom(format!("invalid DNS address {entry:?}: {e}")))
            })
            .collect::<Result<BoxList<IpAddr>, _>>()
            .map(Self)
    }
}

impl std::fmt::Display for DnsServers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "—");
        }

        for (index, ip) in self.0.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{ip}")?;
        }

        Ok(())
    }
}

/// WAN connection state (`ppp_status`).
#[derive(Debug, Clone)]
pub enum PppStatus {
    Connected,
    Disconnected,
    Other(BoxStr),
}

impl<'de> Deserialize<'de> for PppStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = BoxStr::deserialize(deserializer)?;
        Ok(match raw.as_ref() {
            "ppp_connected" => Self::Connected,
            "ppp_disconnected" => Self::Disconnected,
            _ => Self::Other(raw),
        })
    }
}

impl std::fmt::Display for PppStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connected => write!(f, "Connected"),
            Self::Disconnected => write!(f, "Disconnected"),
            Self::Other(status) => write!(f, "{status}"),
        }
    }
}

/// Wi-Fi channel bandwidth, derived from `wifi_11n_cap` the same way the
/// router's own web UI does.
#[derive(Debug, Clone, Copy)]
pub enum ChannelBandwidth {
    Mhz20,
    Mhz20Or40,
    Mhz40,
}

impl<'de> Deserialize<'de> for ChannelBandwidth {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = BoxStr::deserialize(deserializer)?;
        Ok(match raw.as_ref() {
            "0" => Self::Mhz20,
            "1" => Self::Mhz20Or40,
            _ => Self::Mhz40,
        })
    }
}

impl std::fmt::Display for ChannelBandwidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mhz20 => write!(f, "20MHz"),
            Self::Mhz20Or40 => write!(f, "20MHz/40MHz"),
            Self::Mhz40 => write!(f, "40MHz"),
        }
    }
}

/// SMS state (`tag` field of an inbox message).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageStatus {
    Read,
    Unread,
    Sent,
    Unknown(BoxStr),
}

impl<'de> Deserialize<'de> for MessageStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = BoxStr::deserialize(deserializer)?;
        Ok(match raw.as_ref() {
            "0" => Self::Read,
            "1" => Self::Unread,
            "2" => Self::Sent,
            _ => Self::Unknown(raw),
        })
    }
}

impl std::fmt::Display for MessageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read => write!(f, "READ"),
            Self::Unread => write!(f, "UNREAD"),
            Self::Sent => write!(f, "SENT"),
            Self::Unknown(tag) => write!(f, "UNKNOWN({tag})"),
        }
    }
}

/// A timestamp in the router's "YY,MM,DD,HH,MM,SS,+TZ" format.
#[derive(Debug, Clone)]
pub struct Datetime {
    datetime: jiff::Zoned,
}

impl Datetime {
    /// The timezone part can be ignored because it is a lie. It
    /// either says +8 or +12, but the actual time is always in GMT+3.
    /// Examples
    /// 26,06,05,13,06,24,+8
    /// 26,06,18,16,35,06,+12
    pub fn parse(datetime: &str) -> EyreResult<Self> {
        let datetime = datetime.trim_end_matches(",+8").trim_end_matches(",+12");
        let datetime = format!("{datetime},Africa/Addis_Ababa");
        let datetime = jiff::Zoned::strptime("%y,%m,%d,%H,%M,%S,%Q", datetime)?;
        Ok(Self { datetime })
    }

    pub fn now() -> Self {
        Self {
            datetime: jiff::Zoned::now(),
        }
    }

    pub fn router_time(&self) -> BoxStr {
        let datetime = self.datetime.strftime("%y;%m;%d;%H;%M;%S");
        format!("{datetime};+3").into_boxed_str()
    }
}

impl<'de> Deserialize<'de> for Datetime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = BoxStr::deserialize(deserializer)?;
        Self::parse(&raw).map_err(|e| D::Error::custom(format!("invalid timestamp {raw:?}: {e}")))
    }
}

impl std::fmt::Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let datetime = self.datetime.strftime("%F %r");
        write!(f, "{datetime}")
    }
}
