use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "command", rename_all = "camelCase")]
pub enum Command {
  #[serde(rename_all = "camelCase")]
  UpdateQuery { query_string: String },
  #[serde(rename_all = "camelCase")]
  AddFolder,
  #[serde(rename_all = "camelCase")]
  RevealFile { file_path: String },
}
