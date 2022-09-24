use actix_web::{get, web, App, HttpServer, Responder};

#[get("/status")]
async fn get_status() -> impl Responder {
  format!("Ok")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
  println!("Server listening in port 8080");
  HttpServer::new(|| {
    App::new()
      .route("/hello", web::get().to(|| async { "Hello World!" }))
      .service(get_status)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
