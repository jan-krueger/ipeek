use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip, QueryOptions};

pub async fn ip_handler(
    req: HttpRequest,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    format_response(query.format.as_deref(), &get_ip_response(&req))
}

pub fn get_ip_response(req: &HttpRequest) -> SimpleResponse {
    let ip = get_ip(&req).to_string();
    SimpleResponse { value: ip }
}