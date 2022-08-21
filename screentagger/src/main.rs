mod app;
mod data;
mod db;
mod file;

fn main() {
  let mut app = app::app::App::new();
  app::window::create_window(&mut app);
}


