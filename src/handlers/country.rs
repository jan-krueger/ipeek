use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::Reader;
use crate::models::{SimpleResponse, QueryOptions};
use crate::util::{format_response, get_info};

pub async fn country_handler(
    req: HttpRequest,
    geo_db: web::Data<Reader<Vec<u8>>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let info = get_info(&req, &geo_db);
    let result = info.country.unwrap_or_else(|| "unknown".to_string());
    let response = SimpleResponse { value: result };
    format_response(query.format.as_deref(), &response)
}
