use crate::format_middleware::Format;
use crate::util::{format_response, get_info};
use crate::AppState;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use std::sync::Arc;

pub async fn all_handler(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    let info = get_info(&req, &state.geo_db).await;
    format_response(req.extensions().get::<Format>().unwrap(), &info)
}
