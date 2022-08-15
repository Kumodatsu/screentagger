#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Tag {
  Literal(String),
}

pub fn literal(tag_name: &str) -> Tag {
  Tag::Literal(String::from(tag_name))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn literal_works() {
    let a = literal("foo");
    let b = Tag::Literal(String::from("foo"));
    assert_eq!(a, b);
  }
}
