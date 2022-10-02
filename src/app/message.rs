use chrono::prelude::*;
use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppMessage {
  pub text: String,
  #[serde(with = "ts_milliseconds", default)]
  pub timestamp: DateTime<Utc>,
  pub username: String,
}

#[cfg(test)]
mod tests {
  use super::AppMessage;
  #[test]
  fn deserialize_json() {
    let str = r#"
    {
      "text": "hello",
      "timestamp": 1664668800000
    }
    "#;

    let msg: AppMessage = serde_json::from_str(str).unwrap();
    assert_eq!(&msg.text, "hello");
    assert_eq!(&msg.timestamp.timestamp_millis(), &1664668800000);
  }
}
