use actix_web::{App, HttpServer, Responder, web};

async fn index() -> impl Responder {
    "Hello, world!\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/app").route("index.html", web::get().to(index)))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
