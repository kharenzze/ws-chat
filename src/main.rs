use actix::{Actor, StreamHandler};
use actix_files::NamedFile;
use actix_web::{
  get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result as ActixResult,
};
use actix_web_actors::ws;
use std::path::PathBuf;

#[get("/status")]
async fn get_status() -> impl Responder {
  format!("Ok")
}

#[get("/app/{filename:.*}")]
async fn static_content(req: HttpRequest) -> ActixResult<NamedFile> {
  let filename: String = req.match_info().query("filename").parse().unwrap();
  let path = PathBuf::from(format!("./static/{filename}"));
  println!("{}", path.to_str().unwrap());
  Ok(NamedFile::open(path)?)
}

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
  type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => ctx.text(text),
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      _ => (),
    }
  }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  let resp = ws::start(MyWs {}, &req, stream);
  println!("{:?}", resp);
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
