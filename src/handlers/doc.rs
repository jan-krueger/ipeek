use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::Reader;
use crate::util::{get_info, is_browser};

pub async fn doc_handler(
    req: HttpRequest,
    geo_db: web::Data<Reader<Vec<u8>>>,
) -> HttpResponse {
    let info = get_info(&req, &geo_db).await;

    let ip_address   = info.ip;
    let remote_host  = info.reverse_dns.unwrap_or_else(|| "unknown".to_string());
    let country_code = info.country.unwrap_or_else(|| "unknown".to_string());

    let (green, cyan, yellow, magenta, reset) = if is_browser(&req) {
        ("", "", "", "", "")
    } else {
        (
            "\x1b[32m", // green
            "\x1b[36m", // cyan
            "\x1b[33m", // yellow
            "\x1b[35m", // magenta
            "\x1b[0m",  // reset
        )
    };

    let ascii_art = format!(
        "{green}\
 (_)               | |    (_)
  _ _ __   ___  ___| | __  _  ___
 | | '_ \\ / _ \\/ _ \\ |/ / | |/ _ \\
 | | |_) |  __/  __/   < _| | (_) |
 |_| .__/ \\___|\\___|_|\\_(_)_|\\___/
   | |
   |_|\
{reset}",
        green = green,
        reset = reset
    );

    // Unified documentation string, with extra example outputs.
    let doc = format!(
        r#"{ascii_art}

---------------
IP Address:        {yellow}{ip_address}{reset}
Remote Host:       {yellow}{remote_host}{reset}
Country Code:      {yellow}{country_code}{reset}

{magenta}Simple cURL API{reset}
---------------
1) Return IP:
   {cyan}$ curl http://ipeek.io/ip{reset}

2) Return Reverse DNS:
   {cyan}$ curl http://ipeek.io/reverse_dns{reset}

3) Return Country:
   {cyan}$ curl http://ipeek.io/country{reset}

4) Return City:
   {cyan}$ curl http://ipeek.io/city{reset}

5) Return Region:
   {cyan}$ curl http://ipeek.io/region{reset}

6) Return ASN:
   {cyan}$ curl http://ipeek.io/asn{reset}

7) Return All Info:
   {cyan}$ curl http://ipeek.io/all{reset}

{magenta}Output Formats{reset}
--------------
By default, responses are returned as plain text.
You can specify a different format using the query parameter 'format':
  - {yellow}json{reset}     : Returns data in JSON format
  - {yellow}xml{reset}      : Returns data in XML format
  - {yellow}csv{reset}      : Returns data in CSV format
  - {yellow}yaml{reset}     : Returns data in YAML format
  - {yellow}msgpack{reset}  : Returns data in MessagePack (binary) format

{magenta}Examples with Outputs{reset}
-------------------------------------
{cyan}$ curl https://ipeek.io/ip?format=json{reset}
{cyan}$ curl https://ipeek.io/all?format=xml{reset}
{cyan}$ curl https://ipeek.io/country?format=csv{reset}
{cyan}$ curl https://ipeek.io/ip?format=yaml{reset}
{cyan}$ curl https://ipeek.io/all?format=msgpack{reset}

"#,
        ascii_art = ascii_art,
        cyan = cyan,
        yellow = yellow,
        magenta = magenta,
        reset = reset,
        ip_address = ip_address,
        remote_host = remote_host,
        country_code = country_code,
    );

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(doc)
}
