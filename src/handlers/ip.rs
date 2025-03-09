use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip};

pub async fn ip_handler(req: HttpRequest) -> HttpResponse {
    format_response(req.extensions().get::<String>().unwrap(), &get_ip_response(&req))
}

pub fn get_ip_response(req: &HttpRequest) -> SimpleResponse {
    let ip = get_ip(&req).to_string();
    SimpleResponse { value: ip }
}
