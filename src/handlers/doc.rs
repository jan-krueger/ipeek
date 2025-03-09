use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use comfy_table::{Attribute, Cell, Color, Table};
use crate::AppState;
use crate::util::{get_info, get_ip, is_browser};

use crate::handlers::asn::get_asn_info;
use crate::handlers::blacklist::check_blacklists;
use crate::models::{AsnRecord, BlacklistReason, Info};

pub async fn doc_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
) -> HttpResponse {
    let info = get_info(&req, &state.geo_db).await;

    let ip_address   = info.ip.clone();
    let remote_host  = info.reverse_dns.clone();
    let country_code = info.country.clone();

    let (green, cyan, yellow, magenta, reset, bold) = if is_browser(&req) {
        ("", "", "", "", "", "")
    } else {
        (
            "\x1b[32m", // green
            "\x1b[36m", // cyan
            "\x1b[33m", // yellow
            "\x1b[35m", // magenta
            "\x1b[0m",  // reset
            "\x1b[1m",  // bold
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

    let doc = format!(
        r#"{ascii_art}

---------------
IP Address:        {yellow}{ip_address}{reset}
Remote Host:       {yellow}{remote_host}{reset}
Country:           {yellow}{country_code}{reset}

{magenta}{bold}Simple cURL API{reset}
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

{magenta}{bold}Output Formats{reset}
--------------
By default, responses are returned as plain text.
You can specify a different format using the query parameter 'format':
  - {yellow}json{reset}     : Returns data in JSON format
  - {yellow}xml{reset}      : Returns data in XML format
  - {yellow}csv{reset}      : Returns data in CSV format
  - {yellow}yaml{reset}     : Returns data in YAML format
  - {yellow}msgpack{reset}  : Returns data in MessagePack (binary) format

{magenta}{bold}Examples with Outputs{reset}
-------------------------------------
{curl_request_table}

"#,
        ascii_art = ascii_art,
        cyan = cyan,
        yellow = yellow,
        magenta = magenta,
        reset = reset,
        bold = bold,
        ip_address = ip_address,
        curl_request_table = curl_request_table(&info, get_asn_info(&req, &state.asn_db), check_blacklists(get_ip(&req)).await, is_browser(&req)),
    );


    HttpResponse::Ok()
        .content_type("text/plain")
        .body(doc)
}

fn curl_request_table(info: &Info, asn: AsnRecord, blacklist_results: HashMap<String, BlacklistReason>, is_browser: bool) -> String {
    let mut table = Table::new();

    table
        .set_header(vec![
            Cell::new("cURL Request").fg(Color::Green).add_attribute(Attribute::Bold),
            Cell::new("Example Output").fg(Color::Green).add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/").fg(Color::Cyan),
            Cell::new(&info.ip).fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/ip").fg(Color::Cyan),
            Cell::new(&info.ip).fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/reverse_dns").fg(Color::Cyan),
            Cell::new(&info.reverse_dns).fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/country").fg(Color::Cyan),
            Cell::new(&info.country).fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/city").fg(Color::Cyan),
            Cell::new(&info.city).fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/region").fg(Color::Cyan),
            Cell::new(&info.region).fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/asn").fg(Color::Cyan),
            Cell::new(format!(
                "ASN: {}\nOrganization: {}",
                asn.autonomous_system_number
                    .map(|num| num.to_string())
                    .unwrap_or_default(),
                asn.autonomous_system_organization
                    .as_ref()
                    .map(String::as_str)
                    .unwrap_or_default()
            ))
            .fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/all").fg(Color::Cyan),
            Cell::new(&format!(
                "IP: {}\nHostname: {}\nCountry: {}\nRegion: {}\nCity: {}",
                info.ip, info.reverse_dns, info.country, info.region, info.city
            ))
                .fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/blacklist").fg(Color::Cyan),
            Cell::new("IP: <ip>\nBlacklisted: <yes/no>\nLists:\n - <blacklist1> (<reason>)\n - <blacklist2> (<reason>)").fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/blacklist").fg(Color::Cyan),
            Cell::new(&format!(
                "IP: {}\nBlacklisted: {}{}",
                info.ip,
                if !blacklist_results.is_empty() { "yes" } else { "no" },
                if !blacklist_results.is_empty() {
                    let lists = blacklist_results
                        .iter()
                        .filter(|(_, reason)| **reason != BlacklistReason::Unknown)
                        .map(|(dnsbl, reason)| format!("\n - {} ({:?})", dnsbl, reason))
                        .collect::<Vec<_>>()
                        .join("");
                    format!("\nLists:{}", lists)
                } else {
                    String::new()
                }
            )).fg(Color::Yellow),
        ])
        .add_row(vec![
            Cell::new("curl ipeek.io/docs").fg(Color::Cyan),
            Cell::new("(Documentation in plain-text format)").fg(Color::Yellow),
        ]);

    if is_browser {
        table.force_no_tty();
    }
    table.to_string()
}

