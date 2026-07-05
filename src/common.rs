pub type BoxStr = Box<str>;
pub type BoxList<T> = Box<[T]>;
pub type EyreResult<T> = color_eyre::Result<T>;

/// Render a value fetched from the router on stdout.
pub trait Show {
    fn show(&self) -> EyreResult<()>;
}

pub fn b64_encode(input: &str) -> BoxStr {
    use base64::Engine;

    base64::prelude::BASE64_STANDARD
        .encode(input)
        .into_boxed_str()
}

pub fn b64_decode(input: &str) -> EyreResult<BoxStr> {
    use base64::Engine;

    let decoded = base64::prelude::BASE64_STANDARD.decode(input)?;
    let decoded = String::from_utf8(decoded)?.into_boxed_str();

    Ok(decoded)
}

pub fn sha256_encode(input: &str) -> BoxStr {
    use sha2::Digest;

    let hash = sha2::Sha256::digest(input);
    hex::encode(hash).into_boxed_str()
}

pub fn ucs2_decode(hex_str: &str) -> EyreResult<BoxStr> {
    let bytes = hex::decode(hex_str)?;
    if bytes.len() % 2 != 0 {
        color_eyre::eyre::bail!("non even length");
    }

    let units: BoxList<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .collect();

    let str = String::from_utf16(&units)?.into_boxed_str();

    Ok(str)
}

pub fn ucs2_encode(s: &str) -> BoxStr {
    let bytes: BoxList<u8> = s.encode_utf16().flat_map(u16::to_be_bytes).collect();
    hex::encode(bytes).into_boxed_str()
}

pub fn create_table() -> comfy_table::Table {
    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL_CONDENSED);
    table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
    table
}

/// Page text through `$PAGER` (default: `less -FRX`) so long output
/// doesn't flood the terminal. Prints directly when stdout isn't a
/// terminal (e.g. piped into grep) or the pager can't be started.
pub fn page_or_print(text: &str) -> EyreResult<()> {
    use std::io::{IsTerminal, Write};
    use std::process::{Command, Stdio};

    if !std::io::stdout().is_terminal() {
        print!("{text}");
        return Ok(());
    }

    let pager = std::env::var("PAGER").unwrap_or_else(|_| "less -FRX".to_owned());
    let mut words = pager.split_whitespace();
    let Some(program) = words.next() else {
        print!("{text}");
        return Ok(());
    };

    let Ok(mut child) = Command::new(program)
        .args(words)
        .stdin(Stdio::piped())
        .spawn()
    else {
        print!("{text}");
        return Ok(());
    };

    if let Some(mut stdin) = child.stdin.take() {
        // A broken pipe just means the user quit the pager early.
        let _ = stdin.write_all(text.as_bytes());
    }

    child.wait()?;

    Ok(())
}

/// Print a block of text bracketed by horizontal rules sized to its
/// widest line, so successive blocks (e.g. USSD menu steps) stay visually
/// separate. The rule width is clamped to keep short replies from drawing
/// a tiny line and long ones from spanning the whole terminal.
pub fn print_framed(text: &str) {
    const MIN_RULE: usize = 24;
    const MAX_RULE: usize = 60;

    let width = text
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
        .clamp(MIN_RULE, MAX_RULE);

    let rule = "─".repeat(width);
    println!("{rule}\n{text}\n{rule}");
}

/// Substitute a dash for fields the router reports as empty.
pub const fn or_dash(value: &str) -> &str {
    if value.is_empty() { "—" } else { value }
}

pub const fn yes_no(value: bool) -> &'static str {
    if value { "Yes" } else { "No" }
}

/// Truncate to `max` characters, appending an ellipsis when cut.
pub fn truncate_chars(text: &str, max: usize) -> BoxStr {
    if text.chars().count() <= max {
        return text.into();
    }

    let truncated: BoxStr = text.chars().take(max).collect();
    format!("{}…", truncated.trim_end()).into_boxed_str()
}
