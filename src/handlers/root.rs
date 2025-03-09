use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use crate::AppState;
use crate::handlers::{doc, ip};
use crate::util::QueryOptions;
use crate::util::is_browser;

pub async fn root_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    if is_browser(&req) {
        doc::doc_handler(req, state).await
    } else {
        ip::ip_handler(req, query).await
    }
}
