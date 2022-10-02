use chrono::prelude::*;
use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppMessage {
  pub text: String,
  #[serde(with = "ts_milliseconds", default)]
  pub timestamp: DateTime<Utc>,
}
