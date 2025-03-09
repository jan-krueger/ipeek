use crate::format_middleware::Format;
use crate::handlers::asn::get_asn_response;
use crate::handlers::blacklist::get_blacklist_response;
use crate::handlers::city::get_city_response;
use crate::handlers::country::get_country_response;
use crate::handlers::country_code::get_country_code_response;
use crate::handlers::ip::get_ip_response;
use crate::handlers::region::get_region_response;
use crate::handlers::reverse_dns::get_reverse_dns_response;
use crate::models::{ToCsv, ToPlainText};
use crate::util::{format_response, get_info, is_browser};
use crate::AppState;
use actix_web::body::MessageBody;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table, TableComponent};
use serde::Serialize;
use std::sync::Arc;

macro_rules! add_row {
    ($table:expr, $endpoint:expr, $resp:expr, $format:expr, $f:expr) => {
        $table.add_row(vec![
            Cell::new("curl").fg(Color::Red),
            Cell::new(endpoint_url($endpoint, $format))
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
            Cell::new($f($format, &$resp)).fg(Color::DarkYellow),
        ])
    };
}

fn endpoint_url(endpoint: &str, format: &Format) -> String {
    if format.to_string().is_empty() {
        format!("ipeek.io/{}", endpoint)
    } else {
        format!("ipeek.io/{}.{}", endpoint, format)
    }
}

pub async fn docs_handler(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    let info = get_info(&req, &state.geo_db).await;

    let ip_address = info.ip.clone();
    let remote_host = info.reverse_dns.clone();
    let country_code = info.country.clone();

    let (green, yellow, magenta, red, cyan, underline, reset, bold) = if is_browser(&req) {
        ("", "", "", "", "", "", "", "")
    } else {
        (
            "\x1b[32m",  // green
            "\x1b[33m",  // yellow
            "\x1b[35m",  // magenta
            "\x1b[31m",  // red
            " \x1b[36m", // cyan
            "\x1b[4m",   // underline
            "\x1b[0m",   // reset
            "\x1b[1m",   // bold
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
You can request different formats by appending the desired extension to the endpoint URL:

  - {yellow}.json{reset}     → Returns data in JSON format
  - {yellow}.xml{reset}      → Returns data in XML format
  - {yellow}.csv{reset}      → Returns data in CSV format
  - {yellow}.yaml{reset}     → Returns data in YAML format
  - {yellow}.msgpack{reset}  → Returns data in MessagePack (binary) format

{magenta}{bold}Examples:{reset}
  {red}curl {bold}{cyan}ipeek.io/ip{reset}          # Plain text
  {red}curl {bold}{cyan}ipeek.io/ip{underline}.json{reset}     # JSON
  {red}curl {bold}{cyan}ipeek.io/ip{underline}.xml{reset}      # XML
  {red}curl {bold}{cyan}ipeek.io/ip{underline}.csv{reset}      # CSV


{magenta}{bold}Enpoints{reset}
-------------------------------------
{curl_request_table}

"#,
        ip_address = ip_address,
        curl_request_table = curl_request_table(req, state).await.as_str(),
        ascii_art = ascii_art,
        yellow = yellow,
        magenta = magenta,
        cyan = cyan,
        underline = underline,
        reset = reset,
        bold = bold,
    );

    HttpResponse::Ok().content_type("text/plain").body(doc)
}

async fn curl_request_table(req: HttpRequest, state: web::Data<Arc<AppState>>) -> String {
    let mut table = Table::new();
    let format = req.extensions().get::<Format>().unwrap().clone();

    table
        .set_header(vec![
            Cell::new(""),
            Cell::new("cURL Request")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new("Example Output")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
        ])
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_style(TableComponent::VerticalLines, ' ');

    table.add_row(vec![
        Cell::new("curl").fg(Color::Red),
        Cell::new("ipeek.io/")
            .fg(Color::Cyan)
            .add_attribute(Attribute::Bold),
        Cell::new(f(&format, &get_ip_response(&req))).fg(Color::DarkYellow),
    ]);

    add_row!(table, "ip", get_ip_response(&req), &format, f);
    add_row!(
        table,
        "reverse_dns",
        get_reverse_dns_response(&req).await,
        &format,
        f
    );
    add_row!(
        table,
        "country",
        get_country_response(&req, &state),
        &format,
        f
    );
    add_row!(
        table,
        "country_code",
        get_country_code_response(&req, &state),
        &format,
        f
    );
    add_row!(table, "city", get_city_response(&req, &state), &format, f);
    add_row!(
        table,
        "region",
        get_region_response(&req, &state),
        &format,
        f
    );
    add_row!(table, "asn", get_asn_response(&req, &state), &format, f);
    add_row!(
        table,
        "all",
        get_info(&req, &state.geo_db).await,
        &format,
        f
    );
    add_row!(
        table,
        "blacklist",
        get_blacklist_response(&req).await,
        &format,
        f
    );

    table.add_row(vec![
        Cell::new("curl").fg(Color::Red),
        Cell::new("ipeek.io/docs")
            .fg(Color::Cyan)
            .add_attribute(Attribute::Bold),
        Cell::new("(Documentation in plain-text format)").fg(Color::DarkYellow),
    ]);

    if is_browser(&req) {
        table.force_no_tty();
    } else {
        table.enforce_styling();
    }
    table.to_string()
}

fn f<T, U>(format: &Format, response: &T) -> String
where
    T: Serialize + ToPlainText + ToCsv<U>,
    U: Serialize,
{
    let http_response = format_response(format, response);

    match http_response.into_body().try_into_bytes() {
        Ok(bytes) => {
            if *format == Format::Msgpack {
                let byte_string = bytes
                    .iter()
                    .map(|byte| format!("0x{:02X} ", byte))
                    .collect::<String>();
                return if byte_string.len() > 32 {
                    format!("{}...", &byte_string[..32])
                } else {
                    byte_string
                };
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
        }
        Err(_) => "<failed to extract response body>".to_string(),
    }
}
