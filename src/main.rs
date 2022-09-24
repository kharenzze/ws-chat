use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
  format!("Hello {name}!")
}

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
      .service(greet)
      .service(get_status)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
