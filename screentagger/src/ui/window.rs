use web_view::{Content};

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
    .user_data(())
    .invoke_handler(|_webview, arg| {
      match arg {
        _ => {},
      };
      Ok(())
    })
    .run()
    .expect("Failed to create window.");
}
