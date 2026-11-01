use super::Router;
use crate::*;
use color_eyre::eyre::{bail, eyre};
use inquire::validator::Validation;
use inquire::{Confirm, InquireError, Password, PasswordDisplayMode, Select, Text};
use serde_json::{Map, Value};
use std::ops::RangeInclusive;

type Params = Map<String, Value>;

const WIFI_SECURITY: &[(&str, &str)] = &[("0", "None"), ("1", "WPA-PSK"), ("4", "WPA2-PSK")];
const WIFI_CHANNELS: RangeInclusive<u8> = 1..=13;
const SSID_LENGTH: RangeInclusive<usize> = 1..=64;
const PASSPHRASE_LENGTH: RangeInclusive<usize> = 8..=32;
const NETWORK_MODES: &[(&str, &str)] = &[("0", "4G/3G"), ("1", "4G only"), ("2", "3G only")];

fn prompt<T>(result: Result<T, InquireError>) -> EyreResult<T> {
    match result {
        Ok(value) => Ok(value),
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
            bail!("cancelled, nothing was changed")
        }
        Err(InquireError::NotTTY) => {
            bail!("no terminal to prompt in, pass the values as arguments (see --help)")
        }
        Err(e) => Err(e.into()),
    }
}

fn confirm(message: &str, yes: bool) -> EyreResult<bool> {
    if yes {
        return Ok(true);
    }

    match Confirm::new(message).with_default(false).prompt() {
        Err(InquireError::NotTTY) => bail!("no terminal to confirm in, pass --yes"),
        result => prompt(result),
    }
}

fn length_error(value: &str, range: &RangeInclusive<usize>) -> Option<String> {
    (!range.contains(&value.chars().count()))
        .then(|| format!("must be {} to {} characters", range.start(), range.end()))
}

fn field<'a>(params: &'a Params, key: &str) -> &'a str {
    params.get(key).and_then(Value::as_str).unwrap_or_default()
}

fn select_code(
    message: &str,
    options: &[(&'static str, &'static str)],
    current: &str,
) -> EyreResult<&'static str> {
    let labels: Vec<&str> = options.iter().map(|(_, label)| *label).collect();
    let cursor = options
        .iter()
        .position(|(code, _)| *code == current)
        .unwrap_or(0);
    let label = prompt(
        Select::new(message, labels)
            .with_starting_cursor(cursor)
            .prompt(),
    )?;
    let (code, _) = options
        .iter()
        .find(|(_, l)| *l == label)
        .expect("label comes from options");
    Ok(code)
}

impl Router {
    async fn params(&self, cmdid: &str) -> EyreResult<Params> {
        self.call::<(), Params>(cmdid, None)
            .await?
            .ok_or_else(|| eyre!("router returned no data for {cmdid:?}"))
    }

    pub async fn edit_wifi(&self, changes: WifiChanges, yes: bool) -> EyreResult<bool> {
        let WifiChanges {
            ssid,
            security,
            passphrase,
            channel,
            broadcast,
        } = changes;
        let interactive = ssid.is_none()
            && security.is_none()
            && passphrase.is_none()
            && channel.is_none()
            && broadcast.is_none();

        let mut wifi = self.params("getwifiinfo").await?;
        let old_ssid = field(&wifi, "wifissid").to_owned();
        let old_auth = field(&wifi, "wifiauth").to_owned();
        let old_pass = field(&wifi, "wifipass").to_owned();
        let old_channel = field(&wifi, "wifichan").to_owned();
        let old_broadcast = wifi
            .get("wifibrocast")
            .and_then(Value::as_bool)
            .unwrap_or(true);

        let ssid = match ssid {
            Some(ssid) => ssid,
            None if interactive => prompt(
                Text::new("SSID:")
                    .with_initial_value(&old_ssid)
                    .with_validator(|s: &str| {
                        Ok(match length_error(s, &SSID_LENGTH) {
                            Some(error) => Validation::Invalid(error.into()),
                            None => Validation::Valid,
                        })
                    })
                    .prompt(),
            )?,
            None => old_ssid.clone(),
        };
        if let Some(error) = length_error(&ssid, &SSID_LENGTH) {
            bail!("SSID {error}");
        }

        let auth = match security {
            Some(security) => security.code(),
            None if interactive => select_code("Security:", WIFI_SECURITY, &old_auth)?,
            None => &old_auth,
        };

        let password = match passphrase {
            _ if auth == "0" => old_pass.clone(),
            Some(passphrase) => passphrase,
            None if interactive => {
                let keep_ok = length_error(&old_pass, &PASSPHRASE_LENGTH).is_none();
                let entered = prompt(
                    Password::new("Password (empty keeps the current one):")
                        .with_display_mode(PasswordDisplayMode::Masked)
                        .without_confirmation()
                        .with_validator(move |s: &str| {
                            Ok(match length_error(s, &PASSPHRASE_LENGTH) {
                                Some(_) if s.is_empty() && keep_ok => Validation::Valid,
                                Some(error) => Validation::Invalid(error.into()),
                                None => Validation::Valid,
                            })
                        })
                        .prompt(),
                )?;
                if entered.is_empty() {
                    old_pass.clone()
                } else {
                    entered
                }
            }
            None => old_pass.clone(),
        };
        if auth != "0"
            && let Some(error) = length_error(&password, &PASSPHRASE_LENGTH)
        {
            bail!("Wi-Fi password {error}, pass a new one with --passphrase");
        }

        let channel = match channel {
            Some(channel) => channel.to_string(),
            None if interactive => {
                let channels: Vec<String> = WIFI_CHANNELS.map(|c| c.to_string()).collect();
                let cursor = channels.iter().position(|c| *c == old_channel).unwrap_or(0);
                prompt(
                    Select::new("Channel:", channels)
                        .with_starting_cursor(cursor)
                        .prompt(),
                )?
            }
            None => old_channel.clone(),
        };

        let broadcast = match broadcast {
            Some(broadcast) => broadcast,
            None if interactive => prompt(
                Confirm::new("Broadcast SSID?")
                    .with_default(old_broadcast)
                    .prompt(),
            )?,
            None => old_broadcast,
        };

        let unchanged = ssid == old_ssid
            && auth == old_auth
            && password == old_pass
            && channel == old_channel
            && broadcast == old_broadcast;
        if unchanged {
            println!("No changes.");
            return Ok(false);
        }

        if !confirm(
            "Apply? Wi-Fi restarts and every connected device drops briefly.",
            yes,
        )? {
            println!("Nothing was changed.");
            return Ok(false);
        }

        wifi.insert("wifissid".into(), ssid.into());
        wifi.insert("wifiauth".into(), auth.into());
        wifi.insert("wifipass".into(), password.into());
        wifi.insert("wifichan".into(), channel.into());
        wifi.insert("wifibrocast".into(), broadcast.into());

        self.call::<_, Value>("setwifiinfo", Some(Value::Object(wifi)))
            .await?;
        println!("Wi-Fi settings applied.");

        Ok(true)
    }

    pub async fn wifi_on(&self) -> EyreResult<()> {
        self.call::<(), Value>("openwifiap", None).await?;
        println!("Wi-Fi turned on.");

        Ok(())
    }

    pub async fn change_password(&self, current: &str, new: Option<String>) -> EyreResult<()> {
        let new = match new {
            Some(new) => new,
            None => prompt(
                Password::new("New admin password:")
                    .with_display_mode(PasswordDisplayMode::Masked)
                    .with_custom_confirmation_message("Confirm new admin password:")
                    .with_validator(|s: &str| {
                        Ok(if s.is_empty() {
                            Validation::Invalid("must not be empty".into())
                        } else {
                            Validation::Valid
                        })
                    })
                    .prompt(),
            )?,
        };
        if new.is_empty() {
            bail!("the new admin password must not be empty");
        }

        let params = serde_json::json!({ "oldPwd": current, "newPwd": new, "confirmPwd": new });
        self.call::<_, Value>("changepwd", Some(params)).await?;
        println!("Admin password changed. Pass the new one with --password from now on.");

        Ok(())
    }

    pub async fn set_network_mode(&self, mode: Option<NetworkModeArg>) -> EyreResult<()> {
        let mut network = self.params("getnetworkmode").await?;
        let current = field(&network, "netmode").to_owned();

        let mode = match mode {
            Some(mode) => mode.code(),
            None => select_code("Network mode:", NETWORK_MODES, &current)?,
        };
        if mode == current {
            println!("No changes.");
            return Ok(());
        }

        network.insert("netmode".into(), mode.into());
        self.call::<_, Value>("setnetworkmode", Some(Value::Object(network)))
            .await?;
        println!("Network mode changed.");

        Ok(())
    }

    pub async fn set_usb_tether(&self, on: bool) -> EyreResult<()> {
        let state = if on { "1" } else { "0" };
        self.call::<_, Value>("sethimiusbtether", Some(state))
            .await?;
        println!("USB tethering turned {}.", if on { "on" } else { "off" });

        Ok(())
    }

    pub async fn set_apn(&self, name: Option<&str>) -> EyreResult<()> {
        let info = self.params("getapninfo").await?;
        let profiles = info
            .get("apnlist")
            .and_then(Value::as_array)
            .filter(|profiles| !profiles.is_empty())
            .ok_or_else(|| eyre!("router returned no APN profiles"))?;
        let current = match info.get("apncurrent") {
            Some(Value::Number(n)) => n.as_u64().unwrap_or(0) as usize,
            Some(Value::String(s)) => s.parse().unwrap_or(0),
            _ => 0,
        };

        let text = |profile: &Value, key: &str| -> String {
            profile
                .get(key)
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned()
        };

        let index = match name {
            Some(name) => profiles
                .iter()
                .position(|p| text(p, "apnname").eq_ignore_ascii_case(name))
                .ok_or_else(|| {
                    let names: Vec<String> = profiles.iter().map(|p| text(p, "apnname")).collect();
                    eyre!(
                        "no APN profile named {name:?}, the profiles are: {}",
                        names.join(", ")
                    )
                })?,
            None => {
                let options: Vec<String> = profiles
                    .iter()
                    .map(|p| {
                        format!(
                            "{} ({}, {})",
                            text(p, "apnname"),
                            text(p, "apn"),
                            text(p, "pdptype")
                        )
                    })
                    .collect();
                prompt(
                    Select::new("APN:", options)
                        .with_starting_cursor(current.min(profiles.len() - 1))
                        .raw_prompt(),
                )?
                .index
            }
        };
        if index == current {
            println!("No changes.");
            return Ok(());
        }

        let profile = &profiles[index];
        self.call::<_, Value>("setcurrentapn", Some(profile))
            .await?;
        println!("Active APN changed to {}.", text(profile, "apnname"));

        Ok(())
    }
}
