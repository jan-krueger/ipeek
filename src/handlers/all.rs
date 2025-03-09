use std::sync::Arc;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use crate::AppState;
use crate::util::{format_response, get_info};

pub async fn all_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
) -> HttpResponse {
    let info = get_info(&req, &state.geo_db).await;
    format_response(req.extensions().get::<String>().unwrap(), &info)
}