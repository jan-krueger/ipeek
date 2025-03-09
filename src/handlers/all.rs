use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use crate::AppState;
use crate::util::{format_response, get_info, QueryOptions};

pub async fn all_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let info = get_info(&req, &state.geo_db).await;
    format_response(query.format.as_deref(), &info)
}