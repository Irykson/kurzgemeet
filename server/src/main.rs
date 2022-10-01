use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

use crate::lib::meetup_service::MeetupService;
use crate::lib::ReadonlyService;

mod lib;
mod services;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/meetups")]
async fn meetups_test() -> impl Responder {
    let mut meetup_service = MeetupService::new();
    let result = meetup_service.get();
    web::Json(json!(result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(meetups_test)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
