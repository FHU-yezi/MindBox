use actix_web::{get, web::Json, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HelloWorldResponse<'a> {
    foo: &'a str,
    secret: u8,
}

#[get("/hello-world")]
async fn hello_world_handler() -> Result<impl Responder, std::io::Error> {
    Ok(Json(HelloWorldResponse {
        foo: "bar",
        secret: 42,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(hello_world_handler))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
