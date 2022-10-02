use actix::prelude::*;

#[derive(Debug, Default)]
pub struct WsServer {
  n_sessions: usize,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {}

impl WsServer {
  pub fn new() -> Self {
    Default::default()
  }
}

impl Actor for WsServer {
  type Context = Context<Self>;
}

impl Handler<Connect> for WsServer {
  type Result = usize;
  fn handle(&mut self, _msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
    self.n_sessions += 1;
    println!("Number of sessions: {}", self.n_sessions);
    0
  }
}
