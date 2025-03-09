use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip, QueryOptions};

pub async fn ip_handler(
    req: HttpRequest,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let ip = get_ip(&req).to_string();
    let response = SimpleResponse { value: ip };
    format_response(query.format.as_deref(), &response)
}
