use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn upload() -> impl Responder {
    HttpResponse::Ok().body("Hi, na?!")
}
async fn download(info: web::Path<(String)>) -> HttpResponse {
    HttpResponse::Ok().body("Wat willst du denn hier?")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/media")
                .route("/img", web::post().to(upload))
                .route("/img/{name}", web::get().to(download))
                .route("/health_check", web::get().to(health_check)),
        )
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
