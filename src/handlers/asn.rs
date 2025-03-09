use std::net::IpAddr;
use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::Reader;
use crate::AppState;
use crate::models::{AsnRecord, AsnResponse};
use crate::util::{format_response, get_ip, QueryOptions};

pub async fn asn_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    format_response(query.format.as_deref(), &get_asn_response(&req, &state))
}

pub fn get_asn_info(req: &HttpRequest, asn_db: &Reader<Vec<u8>>) -> AsnRecord {
    let ip: IpAddr = get_ip(&req);

    asn_db.lookup(ip).unwrap_or(AsnRecord {
        autonomous_system_number: None,
        autonomous_system_organization: None,
    })
}

pub fn get_asn_response(req: &HttpRequest, state: &web::Data<Arc<AppState>>) -> AsnResponse {
    let asn_info = get_asn_info(&req, &state.asn_db);
    AsnResponse {
        autonomous_system_number: asn_info.autonomous_system_number,
        autonomous_system_organization: asn_info.autonomous_system_organization,
    }
}
