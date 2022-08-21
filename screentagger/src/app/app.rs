use crate::data::parse::parse_query;
use crate::data::tag::literal;
use crate::db::database::Database;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct App {
  db: Database,
}

impl App {
  pub fn new() -> Self {
    let app = App {
      db: Database::new(":memory:"),
    };
    app.db.add_tags(&HashSet::from([literal("image")]));
    app
  }

  pub fn query_prompt(&mut self, query_string: &str) -> HashSet<String> {
    if let Ok(query) = parse_query(query_string) {
      self.db.query_files(&query)
    } else {
      HashSet::from([])
    }
  }

  pub fn add_folder<P: AsRef<Path>>(&self, path: P) {
    let files = fs::read_dir(path)
      .expect("Invalid folder.")
      .filter_map(|entry| if let Ok(entry) = entry {
        Some(String::from(entry.path().to_str().expect("Invalid path.")))
      } else {
        None 
      })
      .collect::<HashSet<_>>();
    self.db.add_files(&files);
    for file in files {
      self.db.add_tags_to_file(file, &HashSet::from([literal("image")]));
    }
  }
}

#[cfg(test)]
mod tests {}
