use crate::format_middleware::Format;
use crate::models::AsnRecord;
use crate::util::{format_response, get_ip};
use crate::AppState;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use maxminddb::Reader;
use std::net::IpAddr;
use std::sync::Arc;

pub async fn asn_handler(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    format_response(
        req.extensions().get::<Format>().unwrap(),
        &get_asn_response(&req, &state),
        false,
    )
}

pub fn get_asn_info(req: &HttpRequest, asn_db: &Reader<Vec<u8>>) -> AsnRecord {
    let ip: IpAddr = get_ip(&req);

    asn_db.lookup(ip).unwrap_or(AsnRecord {
        autonomous_system_organization: Some(0),
        autonomous_system_number: Some("".to_string()),
    })
}

pub fn get_asn_response(req: &HttpRequest, state: &web::Data<Arc<AppState>>) -> AsnRecord {
    let asn_info = get_asn_info(&req, &state.asn_db);
    AsnRecord {
        autonomous_system_organization: asn_info.autonomous_system_organization,
        autonomous_system_number: asn_info.autonomous_system_number,
    }
}
