extern crate chrono;
mod app;

use crate::app::ws::AppWs;

use actix_files::NamedFile;
use actix_web::{
  get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result as ActixResult,
};
use actix_web_actors::ws;
use std::path::PathBuf;

const DEBUG_STATIC: bool = false;
const DEBUG_WS_TREAM: bool = false;

#[get("/status")]
async fn get_status() -> impl Responder {
  format!("Ok")
}

#[get("/app/{filename:.*}")]
async fn static_content(req: HttpRequest) -> ActixResult<NamedFile> {
  let filename: String = req.match_info().query("filename").parse().unwrap();
  let path = PathBuf::from(format!("./static/{filename}"));
  if DEBUG_STATIC {
    println!("{}", path.to_str().unwrap());
  }
  Ok(NamedFile::open(path)?)
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  let resp = ws::start(AppWs {}, &req, stream);
  if DEBUG_WS_TREAM {
    println!("{:?}", resp);
  }
  resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Server listening in port 8080");
  HttpServer::new(|| {
    App::new()
      .route("/ws/", web::get().to(index))
      .service(get_status)
      .service(static_content)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
