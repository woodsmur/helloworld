use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn pong() -> impl Responder {
    let msg = Message {
        message: "pong".into(),
    };
    let msg = serde_json::to_string(&msg).unwrap();
    HttpResponse::Ok().body(&msg)
}

async fn pongtext() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(pong))
            .route("/pingtext", web::get().to(pongtext))
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
