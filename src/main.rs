use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, body::BoxBody, get,
    http::header::ContentType,
};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body + "\n")
    }
}

#[get("/")]
async fn index() -> impl Responder {
    MyObj { name: "Andrew" }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8081")?
        .run()
        .await
}
