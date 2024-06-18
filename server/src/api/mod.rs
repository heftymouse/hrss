use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/")]
pub async fn do_thing() -> HttpResponse {
    HttpResponse::Ok().body("aaaaaa")
}