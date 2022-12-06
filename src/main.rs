// use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::net::TcpListener;
use zero2prod::run;
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // HttpServer::new(|| {
    //     App::new()
    //         .route("/", web::get().to(greet))
    //         .route("/{name}", web::get().to(greet))
    // })
    // .bind("127.0.0.1:8000")?
    // .run()
    // .await
    run(listener)?.await
}
