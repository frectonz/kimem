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
