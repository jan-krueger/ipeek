use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::{SimpleResponse, QueryOptions};
use crate::util::{format_response, get_ip};

pub async fn ip_handler(
    req: HttpRequest,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let ip = get_ip(&req).to_string();
    let response = SimpleResponse { value: ip };
    format_response(query.format.as_deref(), &response)
}
