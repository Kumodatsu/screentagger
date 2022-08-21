use crate::data::tag::{Tag, literal};
use crate::data::query::{Query, satisfies};
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
        name TEXT    NOT NULL UNIQUE
      );
      CREATE TABLE IF NOT EXISTS tag (
        id   INTEGER NOT NULL PRIMARY KEY,
        name TEXT    NOT NULL UNIQUE
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

  #[allow(dead_code)]
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
            format!("INSERT OR IGNORE INTO tag (name) VALUES ('{}');", name)
          ).expect("Failed to insert tag.");
        },
      }
    }
  }

  pub fn add_files(&self, files: &HashSet<String>) {
    for file in files {
      self.connection.execute(
        format!("INSERT OR IGNORE INTO file (name) VALUES ('{}');", file)
      ).expect("Failed to insert file.");
    }
  }

  pub fn add_tags_to_file(&self, file: String, tags: &HashSet<Tag>) {
    let file_id = self.get_file_id(&file);
    for tag in tags {
      let tag_id = self.get_tag_id(tag);
      self.connection
        .execute(
          format!("
            INSERT OR IGNORE INTO file_tag (file_id, tag_id)
            VALUES ({}, {});
          ", file_id, tag_id)
        )
        .expect("Failed create file selection statement.");
    }
  }

  pub fn get_tags_for_file(&self, file: &str) -> HashSet<Tag> {
    self.connection
      .prepare(
        format!("
          SELECT tag.name
          FROM file
            JOIN file_tag ON file.id = file_tag.file_id
            JOIN tag      ON tag.id  = file_tag.tag_id
          WHERE file.name = '{}'
          ORDER BY tag.name ASC
        ", file)
      )
      .expect("Failed create file selection statement.")
      .into_cursor()
      .filter_map(|res| {
        if let Ok(row) = res {
          Some(literal(&row.get::<String, _>(0)))
        } else {
          None
        }
      })
      .collect::<HashSet<_>>()
  }

  pub fn query_files(&self, query: &Query) -> HashSet<String> {
    self.connection.prepare("SELECT name FROM file")
      .expect("Failed to create file selection statement.")
      .into_cursor()
      .filter_map(|res| {
        if let Ok(row) = res {
          let file_name = row.get::<String, _>(0);
          let tags      = self.get_tags_for_file(&file_name);
          if satisfies(&tags, &query) {
            return Some(file_name);
          }
        }
        None
      })
      .collect::<HashSet<_>>()
  }

  fn get_file_id(&self, file: &str) -> usize {
    self.connection
      .prepare(
        format!("SELECT id FROM file WHERE name = '{}'", file)
      )
      .expect("Failed to create file selection statement.")
      .into_cursor()
      .next()
      .expect("Queried file not present in database.")
      .expect("Failed to get id from file.")
      .get::<i64, _>(0)
      as usize
  }

  fn get_tag_id(&self, tag: &Tag) -> usize {
    let Tag::Literal(tag_name) = tag;
    self.connection
      .prepare(
        format!("SELECT id FROM tag WHERE name = '{}'", tag_name)
      )
      .expect("Failed to create tag selection statement.")
      .into_cursor()
      .next()
      .expect("Queried tag not present in database.")
      .expect("Failed to get id from tag.")
      .get::<i64, _>(0)
      as usize
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
    
    let db_files = db.connection
      .prepare("SELECT name FROM file")
      .unwrap()
      .into_cursor()
      .map(|row| { row.unwrap().get::<String, _>(0) })
      .collect::<HashSet<_>>();

    assert_eq!(files, db_files);
  }

  #[test]
  fn tags_can_be_added_to_files() {
    let db = Database::new(":memory:");
    let files = HashSet::from([
      String::from("foo.jpg"),
      String::from("bar.jpg"),
      String::from("bla.jpg"),
    ]);
    let tags = HashSet::from([
      literal("foo"),
      literal("bar"),
      literal("bla"),
    ]);

    db.add_files(&files);
    db.add_tags(&tags);

    db.add_tags_to_file(
      String::from("foo.jpg"),
      &HashSet::from([
        literal("foo"),
        literal("bla"),
      ]),
    );

    let db_assoc_count = db.connection
      .prepare("SELECT file_id, tag_id FROM file_tag")
      .unwrap()
      .into_cursor()
      .count();
    
    assert_eq!(db_assoc_count, 2);
  }

  #[test]
  fn tags_can_be_retrieved_for_files() {
    let db = Database::new(":memory:");
    let files = HashSet::from([
      String::from("foo.jpg"),
      String::from("bar.jpg"),
      String::from("bla.jpg"),
    ]);
    let tags = HashSet::from([
      literal("foo"),
      literal("bar"),
      literal("bla"),
    ]);

    db.add_files(&files);
    db.add_tags(&tags);

    let foo_tags = HashSet::from([
      literal("foo"),
      literal("bla"),
    ]);

    db.add_tags_to_file(
      String::from("foo.jpg"),
      &foo_tags,
    );

    let tags = db.get_tags_for_file("foo.jpg");
    assert_eq!(tags, foo_tags);
  }

  #[test]
  fn files_can_be_queried() {
    let db = Database::new(":memory:");
    let files = HashSet::from([
      String::from("foo.jpg"),
      String::from("bar.jpg"),
    ]);
    let tags = HashSet::from([
      literal("foo"),
      literal("bla"),
    ]);

    db.add_files(&files);
    db.add_tags(&tags);

    db.add_tags_to_file(
      String::from("foo.jpg"),
      &HashSet::from([
        literal("foo"),
        literal("bla"),
      ]),
    );

    let expected     = HashSet::from([String::from("foo.jpg")]);
    let query_result = db.query_files(&Query::Tag(literal("foo")));
    assert_eq!(query_result, expected);
  }
}
