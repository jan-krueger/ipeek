use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::Reader;
use crate::models::{AsnResponse, QueryOptions};
use crate::util::{format_response, get_asn_info};

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
