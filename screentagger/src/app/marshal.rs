use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "command", rename_all = "camelCase")]
pub enum Command {
  #[serde(rename_all = "camelCase")]
  UpdateQuery { query_string: String, },
}
