mod data;
mod db;

use std::collections::HashSet;
use data::tag::literal;

fn main() {
  let database = db::database::Database::new(":memory:");

  database.add_files(&HashSet::from([
    String::from("meow.jpg"),
    String::from("rawr.jpg"),
  ]));

  database.add_tags(&HashSet::from([
    literal("cat"),
    literal("dragon"),
    literal("image"),
  ]));

  database.display_table("file");
  database.display_table("tag");
}


