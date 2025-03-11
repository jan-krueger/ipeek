use crate::format_middleware::Format;
use crate::models::{ToCsv, ToPlainText};
use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr};

pub fn format_response<T, U>(format: &Format, data: &T, pretty: bool) -> HttpResponse
where
    T: Serialize + ToPlainText + ToCsv<U> + yaserde::YaSerialize,
    U: Serialize,
{
    match format {
        Format::Json => {
            let json_str = if pretty {
                serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".to_string())
            } else {
                serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string())
            };

            HttpResponse::Ok()
                .content_type("application/json")
                .body(format!("{}\n", json_str))
        }
        Format::Xml => {
            let yaserde_cfg = yaserde::ser::Config {
                perform_indent: pretty,
                ..Default::default()
            };
            match yaserde::ser::to_string_with_config(data, &yaserde_cfg) {
                Ok(xml_str) => HttpResponse::Ok()
                    .content_type("application/xml")
                    .body(format!("{}\n", xml_str)),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Format::Csv => {
            let mut wtr = csv::Writer::from_writer(vec![]);
            if let Err(err) = data
                .to_csv_entries()
                .iter()
                .try_for_each(|entry| wtr.serialize(entry))
            {
                return HttpResponse::InternalServerError()
                    .body(format!("CSV serialization error: {}", err));
            }

            if let Err(err) = wtr.flush() {
                return HttpResponse::InternalServerError()
                    .body(format!("CSV flush error: {}", err));
            }

            let csv_data = String::from_utf8(wtr.into_inner().unwrap_or_default())
                .unwrap_or_else(|_| "CSV encoding error".to_string());

            HttpResponse::Ok().content_type("text/csv").body(csv_data)
        }
        Format::Yaml => match serde_yml::to_string(data) {
            Ok(yaml_str) => HttpResponse::Ok()
                .content_type("application/x-yaml")
                .body(yaml_str),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Format::Msgpack => match rmp_serde::to_vec(data) {
            Ok(bin_data) => HttpResponse::Ok()
                .content_type("application/msgpack")
                .body(bin_data),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Format::Plain => HttpResponse::Ok()
            .content_type("text/plain")
            .body(format!("{}\n", data.to_plain_text())),
    }
}

pub fn get_ip(req: &HttpRequest) -> IpAddr {
    if let Some(forwarded_for) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_for_str) = forwarded_for.to_str() {
            if let Some(ip) = forwarded_for_str.split(',').next() {
                let ip_trimmed = ip.trim();
                if let Ok(parsed_ip) = ip_trimmed.parse::<IpAddr>() {
                    return parsed_ip;
                }
            }
        }
    }

    if let Some(real_ip) = req.headers().get("X-Real-IP") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            if let Ok(parsed_ip) = real_ip_str.trim().parse::<IpAddr>() {
                return parsed_ip;
            }
        }
    }

    req.connection_info()
        .realip_remote_addr()
        .and_then(|ip| ip.parse::<IpAddr>().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

static KNOWN_BROWSERS: [&str; 8] = [
    "chrome",
    "firefox",
    "safari",
    "edge",
    "opera",
    "chromium",
    "msie",
    "internet explorer",
];

pub fn is_browser(req: &HttpRequest) -> bool {
    if let Some(ua_value) = req.headers().get("User-Agent") {
        if let Ok(ua_str) = ua_value.to_str() {
            let ua_str_lower = ua_str.to_ascii_lowercase();
            return KNOWN_BROWSERS.iter().any(|b| ua_str_lower.contains(b));
        }
    }
    false
}

static API_TOOLS: [&str; 8] = [
    "postman",
    "httpie",
    "insomnia",
    "swagger",
    "apifox",
    "soapui",
    "paw/",
    "rest-client",  // VS Code REST Client
];

pub fn client_supports_color(req: &HttpRequest) -> bool {
    if is_browser(req) {
        return false;
    }

    if let Some(ua_value) = req.headers().get("User-Agent") {
        if let Ok(ua_str) = ua_value.to_str() {
            let ua_str_lower = ua_str.to_ascii_lowercase();
            if API_TOOLS.iter().any(|tool| ua_str_lower.contains(tool)) {
                return false;
            }
        }
    }

    if let Some(host) = req.headers().get("Host") {
        if let Ok(host_str) = host.to_str() {
            match host_str {
                "pie.dev" => return false,        // HTTPie
                "api.hoppscotch.io" => return false,  // Hoppscotch
                "thunder-client.com" => return false, // Thunder Client (VS Code)
                _ => {}
            }
        }
    }

    if req.headers().get("X-Insomnia-Client").is_some() ||     // Insomnia
       req.headers().get("X-Postman-Interceptor").is_some() ||  // Postman Interceptor
       req.headers().get("Hoppscotch-Origin").is_some() ||     // Hoppscotch
       req.headers().get("X-Bruno").is_some()                   // Bruno
    {
        return false;
    }

    true
}
