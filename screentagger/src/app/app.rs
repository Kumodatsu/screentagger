use crate::db::database::Database;

pub struct App {
  db: Database,
}

impl App {
  pub fn new() -> Self {
    App {
      db: Database::new(":memory:"),
    }
  }

  pub fn query_prompt(&mut self, query_string: &str) {
    println!("{}", query_string);
  }
}
