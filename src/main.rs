use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!\n")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("{req_body}\n"))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey, there!\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
