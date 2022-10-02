use crate::app::message::AppMessage;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use chrono::prelude::*;

pub struct WsSession;

/// Define HTTP actor
impl Actor for WsSession {
  type Context = ws::WebsocketContext<Self>;
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
