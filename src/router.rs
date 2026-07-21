use crate::*;
use color_eyre::eyre::{WrapErr, bail, eyre};
use reqwest::header::REFERER;
use serde::Serialize;
use std::net::IpAddr;

#[derive(Debug, Serialize)]
#[serde(tag = "isTest", rename = "false")]
struct ParamsBody<T> {
    cmd: BoxStr,
    #[serde(flatten)]
    payload: T,
}

#[derive(Debug, Serialize)]
#[serde(tag = "isTest", rename = "false")]
struct FormBody<T> {
    #[serde(rename = "goformId")]
    goform_id: BoxStr,
    #[serde(flatten)]
    payload: T,
}

/// Parse a router response, tolerating the duplicate JSON keys some reads
/// (`home_get`, `system_status`) produce: round-tripping through `Value`
/// keeps the last occurrence instead of failing deserialization.
async fn parse_json<T: serde::de::DeserializeOwned>(response: reqwest::Response) -> EyreResult<T> {
    let text = response.text().await?;

    let value: serde_json::Value = serde_json::from_str(&text)
        .wrap_err_with(|| format!("router returned non-JSON response: {text}"))?;

    serde_json::from_value(value)
        .wrap_err_with(|| format!("unexpected response shape (session expired?): {text}"))
}

pub struct Router {
    client: reqwest::Client,
    address: BoxStr,
    username: BoxStr,
    password: BoxStr,
}

impl Router {
    pub fn new(router: &IpAddr, username: &str, password: &str) -> EyreResult<Self> {
        Ok(Self {
            client: reqwest::Client::builder().user_agent("Kimem CLI").build()?,
            address: format!("http://{router}").into_boxed_str(),
            username: username.into(),
            password: password.into(),
        })
    }

    async fn fetch(&self, url: &str) -> EyreResult<reqwest::Response> {
        with_retry(|| async { self.client.get(url).send().await.map_err(Into::into) }).await
    }

    pub async fn get_with<Req: ProcGet>(&self, params: Req::Params) -> EyreResult<Req> {
        let address = self.address.as_ref();

        let params = serde_urlencoded::to_string(&ParamsBody {
            cmd: Req::CMD.into(),
            payload: params,
        })?;

        let url = format!("{address}/reqproc/proc_get?{params}");
        parse_json(self.fetch(&url).await?).await
    }

    pub async fn get<Req: ProcGet>(&self) -> EyreResult<Req> {
        self.get_with::<Req>(Req::Params::default()).await
    }

    pub async fn get_multi<Req: ProcGetMulti>(&self) -> EyreResult<Req> {
        let address = self.address.as_ref();
        let cmd = Req::CMDS.join(",");

        let url = format!("{address}/reqproc/proc_get?cmd={cmd}&multi_data=1&isTest=false");
        parse_json(self.fetch(&url).await?).await
    }

    pub async fn create_post<Req: ProcPost>(
        &self,
        params: Req::Params,
    ) -> EyreResult<reqwest::Response> {
        let address = self.address.as_ref();
        let url = format!("{address}/reqproc/proc_post");

        let form = FormBody {
            goform_id: Req::GOFORM_ID.into(),
            payload: params,
        };

        with_retry(|| async {
            self.client
                .post(&url)
                .form(&form)
                .header(REFERER, self.address.as_ref())
                .send()
                .await
                .map_err(Into::into)
        })
        .await
    }

    pub async fn post_with<Req: ProcPost>(&self, params: Req::Params) -> EyreResult<Req> {
        parse_json(self.create_post::<Req>(params).await?).await
    }

    pub async fn login(&self) -> EyreResult<()> {
        let password = self.password.as_ref();
        let random_login = self.get::<GetRandomLogin>().await?;
        let nonce = random_login.random_login;

        let form = LoginParams {
            username: b64_encode(&self.username),
            password: b64_encode(&sha256_encode(&format!("{nonce}{password}"))),
            unique_login_credentials: "1".into(),
        };

        let login = self.post_with::<Login>(form).await?;
        match login.result {
            LoginResult::Success => Ok(()),
            LoginResult::InvalidCredentials => bail!("login failed: wrong username or password"),
            LoginResult::MalformedRequest => bail!("login failed: malformed login request"),
            LoginResult::Other(code) => bail!("login failed with result code {code:?}"),
        }
    }

    pub async fn logout(&self) -> EyreResult<()> {
        // The response body is flaky (sometimes empty, sometimes JSON)
        // and we don't act on it either way, so only transport errors
        // matter here.
        self.create_post::<Logout>(()).await?;
        Ok(())
    }

    pub async fn reboot(&self) -> EyreResult<()> {
        // The router kills the connection mid-reboot, so an error here is
        // the expected outcome.
        let _ = self.create_post::<RebootDevice>(()).await;
        println!("Device rebooting.");
        Ok(())
    }

    pub async fn show<T: ProcGet + Show>(&self) -> EyreResult<()> {
        self.get::<T>().await?.show()
    }

    pub async fn show_multi<T: ProcGetMulti + Show>(&self) -> EyreResult<()> {
        self.get_multi::<T>().await?.show()
    }

    pub async fn execute<T: ProcPost + Show>(&self, params: T::Params) -> EyreResult<()> {
        self.post_with::<T>(params).await?.show()
    }

    /// Run an interactive USSD session: dial the code, print each network
    /// response, and read menu replies from stdin until EOF / "q" / an
    /// empty line. The network session is always cancelled on the way out.
    pub async fn ussd_session(&self, code: &str) -> EyreResult<()> {
        self.post_with::<UssdProcess>(UssdParams::send(code))
            .await?;

        let dialog = self.ussd_dialog().await;
        let _ = self.post_with::<UssdProcess>(UssdParams::Cancel).await;

        dialog
    }

    async fn ussd_dialog(&self) -> EyreResult<()> {
        use std::io::IsTerminal;

        self.ussd_print_response().await?;

        if std::io::stdin().is_terminal() {
            self.ussd_dialog_interactive().await
        } else {
            self.ussd_dialog_piped().await
        }
    }

    async fn ussd_dialog_interactive(&self) -> EyreResult<()> {
        use rustyline::error::ReadlineError;

        let mut editor = rustyline::DefaultEditor::new()?;
        loop {
            let line = match editor.readline("> ") {
                Ok(line) => line,
                // Ctrl-C / Ctrl-D quit the dialog, not the process, so
                // the session still gets cancelled on the way out.
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
                Err(e) => return Err(e.into()),
            };

            let reply = line.trim();
            if reply.is_empty() || reply == "q" {
                break;
            }

            let _ = editor.add_history_entry(reply);
            self.ussd_reply(reply).await?;
        }

        Ok(())
    }

    /// Piped stdin (`echo 2 | kimem post ussd menu`): no prompts, no
    /// line editing, just replies.
    async fn ussd_dialog_piped(&self) -> EyreResult<()> {
        use std::io::BufRead;

        for line in std::io::stdin().lock().lines() {
            let line = line?;
            let reply = line.trim();
            if reply.is_empty() || reply == "q" {
                break;
            }

            self.ussd_reply(reply).await?;
        }

        Ok(())
    }

    async fn ussd_reply(&self, reply: &str) -> EyreResult<()> {
        self.post_with::<UssdProcess>(UssdParams::reply(reply))
            .await?;
        self.ussd_print_response().await
    }

    /// Poll until the network answers, then print the decoded response.
    async fn ussd_print_response(&self) -> EyreResult<()> {
        const MAX_POLLS: u32 = 30;

        let mut polls = 0;
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            match self.get::<UssdWriteFlag>().await?.ussd_write_flag {
                UssdFlag::Ready => break,
                UssdFlag::Failed(reason) => bail!("USSD request failed: {reason}"),
                UssdFlag::Pending => {
                    polls += 1;
                    if polls == MAX_POLLS {
                        bail!("USSD request timed out after {MAX_POLLS}s");
                    }
                }
            }
        }

        let response = self.get::<UssdData>().await?;
        print_framed(response.text.trim());

        Ok(())
    }

    /// Signal metrics come from `system_status`, but TAC and EARFCN only
    /// exist as standalone cmds; join the two reads into one report.
    pub async fn show_signal(&self) -> EyreResult<()> {
        let signal = self.get::<Signal>().await?;
        let cell = self.get_multi::<CellExtras>().await?;

        SignalReport { signal, cell }.show()
    }

    pub async fn show_internet(&self) -> EyreResult<()> {
        let internet = self.get_multi::<Internet>().await?;
        let wan = self.get::<WanDetails>().await?;

        InternetReport { internet, wan }.show()
    }

    /// Device identity comes from `home_get`, but the SIM ICCID only
    /// exists as a standalone cmd; join the two reads into one report.
    pub async fn show_device(&self) -> EyreResult<()> {
        let device = self.get::<Device>().await?;
        let sim = self.get::<SimIccid>().await?;

        DeviceReport { device, sim }.show()
    }

    /// The syslog lives on its own endpoint and comes back as plain text.
    pub async fn show_syslog(&self) -> EyreResult<()> {
        let address = self.address.as_ref();
        let url = format!("{address}/data/syslog.html?uniquelogincredentials=1&isTest=false");

        let logs = self.fetch(&url).await?.text().await?;
        page_or_print(&logs)
    }

    /// There is no per-message read, so fetch the inbox and pick the
    /// requested message out of it.
    pub async fn show_sms(&self, msg_id: usize) -> EyreResult<()> {
        let inbox = self.get::<SmsInbox>().await?;

        inbox
            .messages
            .iter()
            .find(|message| message.id == msg_id)
            .ok_or_else(|| eyre!("no message with id {msg_id}"))?
            .show()
    }

    pub async fn delete_all_sms(&self) -> EyreResult<()> {
        let messages = self.get::<SmsInbox>().await?.messages;
        if messages.is_empty() {
            println!("Inbox is empty.");
            return Ok(());
        }

        let msg_id = join_msg_ids(messages.iter().map(|message| message.id));
        self.execute::<DeleteSms>(DeleteSmsParams { msg_id }).await
    }

    pub async fn mark_all_sms(&self) -> EyreResult<()> {
        let messages = self.get::<SmsInbox>().await?.messages;

        let unread = messages
            .iter()
            .filter(|message| message.tag == MessageStatus::Unread)
            .map(|message| message.id);
        let msg_id = join_msg_ids(unread);

        if msg_id.is_empty() {
            println!("No unread messages.");
            return Ok(());
        }

        self.execute::<MarkSms>(MarkSmsParams::new(msg_id)).await
    }
}
