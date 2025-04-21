use actix_web::{App, HttpResponse, HttpServer, web};

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("/app\n") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("/test\n") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::to(|| async { HttpResponse::Ok().body("/\n") }))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
