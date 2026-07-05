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
kimem post ussd <code>                   Start an interactive USSD session
kimem post sms send <number> <message>   Send an SMS message
kimem post sms delete <msg_id>|all       Delete message(s) from the inbox
kimem post sms mark <msg_id>|all         Mark message(s) as read
```

The router address and credentials default to `192.168.0.1` / `admin` /
`admin` and can be overridden with `--router`, `--username`, and
`--password`.

## Examples

```sh
# Read grouped status
kimem get info
kimem get signal

# Read one SMS, then mark everything read
kimem get sms show 715
kimem post sms mark all

# Send an SMS
kimem post sms send +251900000000 "hello world"

# Dial a raw code (quote it) or a built-in name
kimem post ussd '*704#'
kimem post ussd balance          # built-ins: menu, balance, bundles, mpesa

# Reply at the `>` prompt, or pipe replies for scripting
echo 2 | kimem post ussd menu

# Talk to a router at a different address
kimem --router 192.168.8.1 --password s3cret get device
```
