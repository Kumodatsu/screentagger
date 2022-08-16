use web_view::{Content};

pub fn create_window() {
  web_view::builder()
    .title("Screentagger")
    .content(Content::Html(include_str!("res/ui.html")))
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
