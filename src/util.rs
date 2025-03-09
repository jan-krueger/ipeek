use actix_web::{HttpRequest, HttpResponse};
use maxminddb::Reader;
use serde::Serialize;
use crate::models::{Info, ToPlainText};
use std::net::{IpAddr, Ipv4Addr};
use crate::handlers::city::get_city;
use crate::handlers::country::get_country;
use crate::handlers::region::get_region;
use crate::handlers::reverse_dns::get_reverse_dns;

pub fn format_response<T>(format: Option<&str>, data: &T) -> HttpResponse
where
    T: Serialize + ToPlainText,
{
    match format {
        Some("json") => {
            let json_str = serde_json::to_string(data)
                .unwrap_or_else(|err| err.to_string());
            HttpResponse::Ok()
                .content_type("application/json")
                .body(format!("{}\n", json_str))
        },
        Some("xml") => match serde_xml_rs::to_string(data) {
            Ok(xml_str) => HttpResponse::Ok()
                .content_type("application/xml")
                .body(format!("{}\n", xml_str)),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Some("csv") => {
            let mut wtr = csv::Writer::from_writer(vec![]);
            if let Err(err) = wtr.serialize(data) {
                return HttpResponse::InternalServerError().body(err.to_string());
            }
            if let Err(err) = wtr.flush() {
                return HttpResponse::InternalServerError().body(err.to_string());
            }
            let csv_data = String::from_utf8(wtr.into_inner().unwrap_or_default())
                .unwrap_or_default();
            HttpResponse::Ok()
                .content_type("text/csv")
                .body(csv_data)
        },
        Some("yaml") => {
            match serde_yml::to_string(data) {
                Ok(yaml_str) => HttpResponse::Ok()
                    .content_type("application/x-yaml")
                    .body(yaml_str),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        },
        Some("msgpack") => {
            match rmp_serde::to_vec(data) {
                Ok(bin_data) => HttpResponse::Ok()
                    .content_type("application/msgpack")
                    .body(bin_data),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        },
        _ => HttpResponse::Ok()
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


pub async fn get_info(req: &HttpRequest, geo_db: &Reader<Vec<u8>>) -> Info {
    let ip: IpAddr = get_ip(req);
    let reverse_dns = get_reverse_dns(ip).await.unwrap_or_else(|| "".to_string());
    let city = get_city(ip, geo_db).unwrap_or_else(|| "".to_string());
    let country = get_country(ip, geo_db).unwrap_or_else(|| "".to_string());
    let region = get_region(ip, geo_db).unwrap_or_else(|| "".to_string());

    Info {
        ip: ip.to_string(),
        reverse_dns,
        country,
        city,
        region,
    }
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
