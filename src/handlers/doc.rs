use crate::util::{format_response, get_info, is_browser, QueryOptions};
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table, TableComponent};
use std::sync::Arc;
use actix_web::body::MessageBody;
use serde::Serialize;
use crate::handlers::asn::get_asn_response;
use crate::handlers::blacklist::get_blacklist_response;
use crate::handlers::city::get_city_response;
use crate::handlers::country::get_country_response;
use crate::handlers::country_code::get_country_code_response;
use crate::handlers::ip::get_ip_response;
use crate::handlers::region::get_region_response;
use crate::handlers::reverse_dns::get_reverse_dns_response;
use crate::models::{ToCsv, ToPlainText};

pub async fn doc_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let info = get_info(&req, &state.geo_db).await;

    let ip_address   = info.ip.clone();
    let remote_host  = info.reverse_dns.clone();
    let country_code = info.country.clone();

    let (green, yellow, magenta, reset, bold) = if is_browser(&req) {
        ("", "", "", "", "")
    } else {
        (
            "\x1b[32m", // green
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
        yellow = yellow,
        magenta = magenta,
        reset = reset,
        bold = bold,
        ip_address = ip_address,
        curl_request_table = curl_request_table(req, state, query).await.as_str(),
    );


    HttpResponse::Ok()
        .content_type("text/plain")
        .body(doc)
}

async fn curl_request_table(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    query: web::Query<QueryOptions>,
) -> String {
    let mut table = Table::new();
    let format = query.format.as_deref();

    table
        .set_header(vec![
            Cell::new("").fg(Color::Green).add_attribute(Attribute::Bold), // Empty for "curl"
            Cell::new("cURL Request").fg(Color::Green).add_attribute(Attribute::Bold),
            Cell::new("Example Output").fg(Color::Green).add_attribute(Attribute::Bold),
        ])
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_style(TableComponent::VerticalLines, ' ');

    // Add rows
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/").fg(Color::Cyan),
        Cell::new(f(format, &get_ip_response(&req))).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/ip").fg(Color::Cyan),
        Cell::new(f(format, &get_ip_response(&req))).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/reverse_dns").fg(Color::Cyan),
        Cell::new(f(format, &get_reverse_dns_response(&req).await)).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/country").fg(Color::Cyan),
        Cell::new(f(format, &get_country_response(&req, &state))).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/country_code").fg(Color::Cyan),
        Cell::new(f(format, &get_country_code_response(&req, &state))).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/city").fg(Color::Cyan),
        Cell::new(f(format, &get_city_response(&req, &state))).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/region").fg(Color::Cyan),
        Cell::new(f(format, &get_region_response(&req, &state))).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/asn").fg(Color::Cyan),
        Cell::new(f(format, &get_asn_response(&req, &state))).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/all").fg(Color::Cyan),
        Cell::new(f(format, &get_info(&req, &state.geo_db).await)).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/blacklist").fg(Color::Cyan),
        Cell::new(f(format, &get_blacklist_response(&req).await)).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("curl").fg(Color::Red).add_attribute(Attribute::Bold),
        Cell::new("ipeek.io/docs").fg(Color::Cyan),
        Cell::new("(Documentation in plain-text format)").fg(Color::Yellow),
    ]);

    if is_browser(&req) {
        table.force_no_tty();
    } else {
        table.enforce_styling();
    }
    table.to_string()
}

fn f<T, U>(format: Option<&str>, response: &T) -> String
    where T: Serialize + ToPlainText + ToCsv<U>,
          U: Serialize
{
    let http_response = format_response(format, response);

    match http_response.into_body().try_into_bytes() {
        Ok(bytes) => {
            if format.as_deref() == Some("msgpack") {
                let byte_string = bytes
                       .iter()
                       .map(|byte| format!("0x{:02X} ", byte))
                       .collect::<String>();
                if byte_string.len() > 32 {
                    return format!("{}...", &byte_string[..32])
                } else {
                    return byte_string
                }
            }

            let full_string = String::from_utf8_lossy(&bytes).to_string();
            full_string
                .lines()
                .map(|line| {
                    if line.len() > 64 {
                        format!("{}...", &line[..64])
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        },
        Err(_) => "<failed to extract response body>".to_string(),
    }
}