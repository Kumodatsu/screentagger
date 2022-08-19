use crate::data::parse::parse_query;
use crate::db::database::Database;
use std::collections::HashSet;

pub struct App {
  db: Database,
}

impl App {
  pub fn new() -> Self {
    App {
      db: Database::new(":memory:"),
    }
  }

  pub fn get_database(&mut self) -> &Database {
    &self.db
  }

  pub fn query_prompt(&mut self, query_string: &str) -> HashSet<String> {
    if let Ok(query) = parse_query(query_string) {
      self.db.query_files(&query)
    } else {
      HashSet::from([])
    }
  }
}

#[cfg(test)]
mod tests {}
