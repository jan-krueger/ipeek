use actix_web::{HttpRequest, HttpResponse};
use maxminddb::Reader;
use serde::Serialize;
use crate::models::{Info, AsnRecord, ToPlainText};
use dns_lookup::lookup_addr;
use std::net::IpAddr;

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
pub fn get_ip(req: &HttpRequest) -> String {
    if let Some(forwarded_for) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_for_str) = forwarded_for.to_str() {
            if let Some(ip) = forwarded_for_str.split(',').next() {
                let ip_trimmed = ip.trim();
                if !ip_trimmed.is_empty() {
                    return ip_trimmed.to_string();
                }
            }
        }
    }

    if let Some(real_ip) = req.headers().get("X-Real-IP") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            if !real_ip_str.trim().is_empty() {
                return real_ip_str.trim().to_string();
            }
        }
    }

    req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string()
}


pub fn get_info(req: &HttpRequest, geo_db: &Reader<Vec<u8>>) -> Info {
    // Parse IP address.
    let ip: IpAddr = get_ip(req).parse().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

    // Reverse DNS lookup.
    let reverse_dns = lookup_addr(&ip).ok();

    // GeoIP lookup.
    let city_record: Option<crate::models::CityRecord> = geo_db.lookup(ip).ok();

    let country = city_record.as_ref().and_then(|record| {
        record.country.as_ref().and_then(|c| {
            c.names.as_ref().and_then(|names| names.get("en").cloned())
        })
    });
    let city = city_record.as_ref().and_then(|record| {
        record.city.as_ref().and_then(|c| {
            c.names.as_ref().and_then(|names| names.get("en").cloned())
        })
    });
    let region = city_record.as_ref().and_then(|record| {
        record.subdivisions.as_ref().and_then(|subs| {
            subs.get(0).and_then(|sub| {
                sub.names.as_ref().and_then(|names| names.get("en").cloned())
            })
        })
    });

    Info {
        ip: ip.to_string(),
        reverse_dns,
        country,
        city,
        region,
    }
}

pub fn get_asn_info(req: &HttpRequest, asn_db: &Reader<Vec<u8>>) -> AsnRecord {
    let connection_info = req.connection_info();
    let ip_str = connection_info.realip_remote_addr().unwrap_or("unknown");
    let ip: IpAddr = ip_str.parse().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

    asn_db.lookup(ip).unwrap_or(AsnRecord {
        autonomous_system_number: None,
        autonomous_system_organization: Some("unknown".to_string()),
    })
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
