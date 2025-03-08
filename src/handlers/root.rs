use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::Reader;
use crate::handlers::{doc, ip};
use crate::models::QueryOptions;
use crate::util::is_browser;

pub async fn root_handler(
    req: HttpRequest,
    geo_db: web::Data<Reader<Vec<u8>>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    if is_browser(&req) {
        doc::doc_handler(req, geo_db).await
    } else {
        ip::ip_handler(req, query).await
    }
}
