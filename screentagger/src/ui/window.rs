use serde_json;
use web_view::{Content};
use crate::ui::app::*;
use crate::ui::marshal::Command;

pub fn create_window() {
  web_view::builder()
    .title("Screentagger")
    .content(Content::Html(format!(
      r#"
      <!doctype html>
      <html>
        <head>
          <style>{css}</style>
        </head>
        <body>{html}</body>
        <script type="text/javascript">{js}</script>
      </html>
      "#,
      css  = include_str!("res/style.css"),
      html = include_str!("res/ui.html"),
      js   = include_str!("res/script.js"),
    )))
    .size(800, 600)
    .resizable(true)
    .debug(true)
    .user_data(App::new())
    .invoke_handler(|webview, arg| {
      let cmd = serde_json::from_str(arg)
        .expect("Failed to deserialize command.");
      match cmd {
        Command::UpdateQuery { query_string } => {
          let app = webview.user_data_mut();
          app.query_prompt(&query_string);
        },
        _ => {},
      };
      Ok(())
    })
    .run()
    .expect("Failed to create window.");
}
