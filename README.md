# Kimem (ቅመም)

Safaricom Kimem MIFI CLI

## Usage

```
kimem get info               Router status information
kimem get system             Router system information
kimem get signal             Router connection signal information
kimem get internet           Router internet connection information
kimem get device             Router device information
kimem get wifi               Router wifi config information
kimem get devices            List connected devices
kimem get sms                List SMS messages
kimem get sms show <msg_id>  Show full SMS message
kimem get syslog             Show router system logs
kimem get airtime            Last cached airtime balance
kimem get power              Router power information

kimem post reboot                        Reboot the router
kimem post sms send <number> <message>   Send an SMS message
kimem post sms delete <msg_id>|all       Delete message(s) from the inbox
kimem post sms mark <msg_id>|all         Mark message(s) as read
```

The router address and credentials default to `192.168.0.1` / `admin` /
`admin` and can be overridden with `--router`, `--username`, and
`--password`.
