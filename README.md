# ipeek.io Documentation

```
 (_)               | |    (_)
  _ _ __   ___  ___| | __  _  ___
 | | '_ \ / _ \/ _ \ |/ / | |/ _ \
 | | |_) |  __/  __/   < _| | (_) |
 |_| .__/ \___|\___|_|\_(_)_|\___/
   | |
   |_|
```

---

## IP Information

- **IP Address:** `192.0.2.1`
- **Remote Host:** `ppp-192-0-2-1.example.com`
- **Country:** `Germany`

## Output Formats

By default, responses are returned as plain text. You can request different formats by appending the desired extension
to the endpoint URL:

- **.json** → Returns data in JSON format
- **.xml** → Returns data in XML format
- **.csv** → Returns data in CSV format
- **.yaml** → Returns data in YAML format
- **.msgpack** → Returns data in MessagePack (binary) format

## Examples

```bash
curl ipeek.io/ip          # Plain text
curl ipeek.io/ip.json     # JSON
curl ipeek.io/ip.xml      # XML
curl ipeek.io/ip.csv      # CSV
```

## IPv4/IPv6 Forcing

You can force an IPv4 connection by using the subdomain `4.ipeek.io` and force an IPv6 connection by using `6.ipeek.io`.

## Endpoints

| cURL Request                 | Example Output                                                                                                             |
|------------------------------|----------------------------------------------------------------------------------------------------------------------------|
| `curl ipeek.io/`             | `192.0.2.1`                                                                                                                |
| `curl ipeek.io/ip`           | `192.0.2.1`                                                                                                                |
| `curl ipeek.io/reverse_dns`  | `ppp-192-0-2-1.example.com`                                                                                                |
| `curl ipeek.io/country`      | `Germany`                                                                                                                  |
| `curl ipeek.io/country_code` | `DE`                                                                                                                       |
| `curl ipeek.io/city`         | `Munich`                                                                                                                   |
| `curl ipeek.io/region`       | `Bavaria`                                                                                                                  |
| `curl ipeek.io/asn`          | `ASN: 8767`<br>`Organization: M-net Telekommunikations GmbH     `                                                          |
| `curl ipeek.io/all`          | `IP: 192.0.2.1`<br>`Hostname: ppp-192-0-2-1.example.com`<br>`Country: Germany (DE)`<br>`Region: Bavaria`<br>`City: Munich` |
| `curl ipeek.io/blacklist`    | `IP: 192.0.2.1`<br>`Blacklisted: yes`<br>`Lists:`<br>&nbsp;&nbsp;&nbsp;&nbsp;- `b.barracudacentral.org (SpamSource)`       |
| `curl ipeek.io/docs`         | (Documentation in plain-text format)                                                                                       |
