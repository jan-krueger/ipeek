use crate::handlers::{docs, ip};
use crate::util::is_browser;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Arc;

pub async fn root_handler(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    if is_browser(&req) {
        docs::docs_handler(req, state).await
    } else {
        ip::ip_handler(req).await
    }
}
