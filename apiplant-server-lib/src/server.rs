use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::start_server]
async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index3)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}