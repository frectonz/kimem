pub type BoxStr = Box<str>;
pub type EyreResult<T> = color_eyre::Result<T>;

pub fn b64_encode(input: &str) -> BoxStr {
    use base64::Engine;

    base64::prelude::BASE64_STANDARD
        .encode(input)
        .into_boxed_str()
}

pub fn b64_decode(input: &str) -> EyreResult<BoxStr> {
    use base64::Engine;

    let decoded = base64::prelude::BASE64_STANDARD.decode(input)?;
    let decocded = String::from_utf8(decoded)?.into_boxed_str();

    Ok(decocded)
}

pub fn sha256_encode(input: &str) -> BoxStr {
    use sha2::Digest;

    let hash = sha2::Sha256::digest(input);
    hex::encode(hash).into_boxed_str()
}

pub fn ucs2_decode(hex_str: &str) -> EyreResult<String> {
    let bytes = hex::decode(hex_str)?;
    if bytes.len() % 2 != 0 {
        color_eyre::eyre::bail!("non even length");
    }

    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .collect();

    let str = String::from_utf16(&units)?;

    Ok(str)
}

pub fn ucs2_encode(s: &str) -> BoxStr {
    let bytes: Vec<u8> = s.encode_utf16().flat_map(|u| u.to_be_bytes()).collect();
    hex::encode(bytes).into_boxed_str()
}

pub fn create_table() -> comfy_table::Table {
    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL_CONDENSED);
    table
}

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
        let datetime = self.datetime.strftime("%y,%m,%d,%H,%M,%S");
        // lets use the chinese timezone for this
        format!("{datetime},+8").into_boxed_str()
    }
}

impl std::fmt::Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let datetime = self.datetime.strftime("%F %r");
        write!(f, "{datetime}")
    }
}
