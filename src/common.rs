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

pub fn create_table() -> comfy_table::Table {
    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL_CONDENSED);
    table
}
