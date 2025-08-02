use std::collections::HashMap;
use serde::Serialize;

use actix_web::{ HttpRequest, HttpResponse, Responder };

/// Returns a HttpResponse::Ok and a payload
///
///

#[derive(Debug, Serialize)]
pub struct HealthyResponse {
    status_code: u16,
    payload: HashMap<String, String>,
}
pub async fn health_check(req: HttpRequest) -> impl Responder {
    let mut payload = HashMap::new();
    payload.insert("message".to_string(), "Service is healthy".to_string());

    let response = HealthyResponse {
        status_code: 200,
        payload: payload,
    };

    HttpResponse::Ok().json(response)
}
