use crate::app::message::AppMessage;
use crate::app::server::{Connect, WsServer};
use actix::prelude::*;
use actix_web_actors::ws;
use chrono::prelude::*;

pub struct WsSession {
  pub addr: Addr<WsServer>,
}

/// Define HTTP actor
impl Actor for WsSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, _ctx: &mut Self::Context) {
    println!("session actor started");
    self
      .addr
      .send(Connect {})
      .into_actor(self)
      .then(|_res, _act, _ctx| fut::ready(()))
      .wait(_ctx);
  }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => {
        let mut msg: AppMessage = serde_json::from_str(&text).unwrap();
        msg.timestamp = Utc::now();
        let json = serde_json::to_string(&msg).unwrap();
        ctx.text(json)
      }
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      _ => (),
    }
  }
}
