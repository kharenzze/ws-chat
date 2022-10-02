use actix::prelude::*;

pub struct WsServer {
  n_sessions: usize,
}
