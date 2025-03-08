use std::net::IpAddr;
use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::Reader;
use crate::models::{AsnRecord, AsnResponse, QueryOptions};
use crate::util::{format_response};

pub async fn asn_handler(
    req: HttpRequest,
    asn_db: web::Data<Reader<Vec<u8>>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let asn_info = get_asn_info(&req, &asn_db);
    let response = AsnResponse {
        autonomous_system_number: asn_info.autonomous_system_number,
        autonomous_system_organization: asn_info.autonomous_system_organization,
    };
    format_response(query.format.as_deref(), &response)
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
