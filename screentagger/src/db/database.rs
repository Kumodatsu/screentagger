use crate::data::tag::Tag;
use std::collections::HashSet;
use sqlite::State;

pub struct Database {
  connection: sqlite::Connection,
}

impl Database {
  pub fn new<T: AsRef<std::path::Path>>(path: T) -> Self {
    let db = Database {
      connection: sqlite::open(path)
        .expect("Failed to open sqlite database.")
    };
    db.connection.execute("
      CREATE TABLE IF NOT EXISTS file (
        id   INTEGER NOT NULL PRIMARY KEY,
        name TEXT NOT NULL
      );
      CREATE TABLE IF NOT EXISTS tag (
        id   INTEGER NOT NULL PRIMARY KEY,
        name TEXT NOT NULL
      );
      CREATE TABLE IF NOT EXISTS file_tag (
        file_id INTEGER NOT NULL,
        tag_id  INTEGER NOT NULL,
        PRIMARY KEY (file_id, tag_id),
        FOREIGN KEY (file_id) REFERENCES file (id),
        FOREIGN KEY (tag_id)  REFERENCES tag  (id)
      );
    ").expect("Failed to create database tables.");
    db
  }

  pub fn display_table(&self, table_name: &str) {
    let mut statement = self.connection
      .prepare(format!("SELECT * FROM {}", table_name))
      .expect("Failed to query table.");
    println!("Table: {}", table_name);
    while let State::Row = statement.next().unwrap() {
      for col in 0 .. statement.column_count() {
        println!("{} = {}",
          statement.column_name(col),
          statement.read::<String>(col).unwrap()
        );
      }
    }
  }

  pub fn add_tags(&self, tags: &HashSet<Tag>) {
    for tag in tags {
      match tag {
        Tag::Literal(name) => {
          self.connection.execute(
            format!("INSERT INTO tag (name) VALUES ('{}');", name)
          ).expect("Failed to insert tag.");
        },
      }
    }
  }

  pub fn add_files(&self, files: &HashSet<String>) {
    for file in files {
      self.connection.execute(
        format!("INSERT INTO file (name) VALUES ('{}');", file)
      ).expect("Failed to insert file.");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::data::tag::literal;

  #[test]
  fn database_creates_tables() {
    let db = Database::new(":memory:");

    let cursor = db.connection.prepare(
      "SELECT name FROM sqlite_master WHERE type='table'"
    ).unwrap().into_cursor();
    let count = cursor.count();

    assert_eq!(count, 3);
  }

  #[test]
  fn added_tags_are_present_in_database() {
    let db = Database::new(":memory:");
    let tags = HashSet::from([
      literal("foo"),
      literal("bar"),
      literal("bla"),
    ]);

    db.add_tags(&tags);
    
    let db_tags = db.connection.prepare("SELECT name FROM tag")
      .unwrap()
      .into_cursor()
      .map(|row| { literal(&row.unwrap().get::<String, _>(0)) })
      .collect::<HashSet<_>>();

    assert_eq!(tags, db_tags);
  }

  #[test]
  fn added_files_are_present_in_database() {
    let db = Database::new(":memory:");
    let files = HashSet::from([
      String::from("foo.jpg"),
      String::from("bar.jpg"),
      String::from("bla.jpg"),
    ]);

    db.add_files(&files);
    
    let db_files = db.connection.prepare("SELECT name FROM file")
      .unwrap()
      .into_cursor()
      .map(|row| { row.unwrap().get::<String, _>(0) })
      .collect::<HashSet<_>>();

    assert_eq!(files, db_files);
  }
}
