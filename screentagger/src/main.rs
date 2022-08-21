mod app;
mod data;
mod db;
mod file;

fn main() {
  let mut app = app::app::App::new();
  let db      = app.get_database();
  db.add_files(&HashSet::from([
    String::from("rawr.jpg"),
    String::from("meow.jpg"),
  ]));
  db.add_tags(&HashSet::from([
    literal("image"),
    literal("dragon"),
    literal("cat"),
  ]));
  db.add_tags_to_file(String::from("rawr.jpg"), &HashSet::from([
    literal("image"),
    literal("dragon"),
  ]));
  db.add_tags_to_file(String::from("meow.jpg"), &HashSet::from([
    literal("image"),
    literal("cat"),
  ]));

  app::window::create_window(&mut app);
}


