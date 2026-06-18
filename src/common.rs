use jiff::Zoned;

pub type BoxStr = Box<str>;
pub type EyreResult<T> = color_eyre::Result<T>;

pub fn b64(input: &str) -> BoxStr {
    use base64::Engine;

    base64::prelude::BASE64_STANDARD
        .encode(input)
        .into_boxed_str()
}

pub fn sha256(input: &str) -> BoxStr {
    use sha2::Digest;

    let hash = sha2::Sha256::digest(input);
    hex::encode(hash).into_boxed_str()
}

pub fn create_table() -> comfy_table::Table {
    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL_CONDENSED);
    table
}

pub fn decode_ucs2_be(hex_str: &str) -> Option<String> {
    let bytes = hex::decode(hex_str).ok()?;
    if bytes.len() % 2 != 0 {
        return None;
    }

    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .collect();

    String::from_utf16(&units).ok()
}

// The timezone part can be ignored because it is a lie. It
// either says +8 or +12, but the actual time is always in GMT+3.
// Examples
// 26,06,05,13,06,24,+8
// 26,06,18,16,35,06,+12
pub fn parse_datetime(datetime: &str) -> EyreResult<Zoned> {
    let datetime = datetime.trim_end_matches(",+8").trim_end_matches(",+12");
    let datetime = format!("{datetime},Africa/Addis_Ababa");
    let zoned = Zoned::strptime("%y,%m,%d,%H,%M,%S,%Q", datetime)?;
    Ok(zoned)
}
