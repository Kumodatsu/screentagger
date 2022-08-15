#[derive(Debug, PartialEq, Eq)]
enum Tag {
  Literal(String),
}

fn literal(tag_name: &str) -> Tag {
  Tag::Literal(String::from(tag_name))
}

#[derive(Debug)]
enum Query {
  Tag(Tag),
  Not(Box<Query>),
  Or(Box<Query>, Box<Query>),
  And(Box<Query>, Box<Query>),
}

fn satisfies(tag_list: &Vec<Tag>, query: Query) -> bool {
  match query {
    Query::Tag(tag)  => tag_list.contains(&tag),
    Query::Not(a)    => !satisfies(tag_list, *a),
    Query::Or(a, b)  => satisfies(tag_list, *a) || satisfies(tag_list, *b),
    Query::And(a, b) => satisfies(tag_list, *a) && satisfies(tag_list, *b),
  }
}

fn main() {
  
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

  #[test]
  fn tag_list_includes_tag() {
    let tag_list = vec![
      literal("foo"),
      literal("bar"),
    ];

    let result = satisfies(&tag_list, Query::Tag(literal("foo")));
    assert_eq!(result, true);
  }

  #[test]
  fn tag_list_does_not_include_tag() {
    let tag_list = vec![
      literal("foo"),
      literal("bar"),
    ];

    let result = satisfies(&tag_list, Query::Tag(literal("bla")));
    assert_eq!(result, false);
  }

  #[test]
  fn tag_list_excludes_tag() {
    let tag_list = vec![
      literal("foo"),
      literal("bar"),
    ];

    let result = satisfies(&tag_list, Query::Not(
      Box::new(Query::Tag(literal("bla")))
    ));
    assert_eq!(result, true);
  }

  #[test]
  fn tag_list_does_not_exclude_tag() {
    let tag_list = vec![
      literal("foo"),
      literal("bar"),
    ];

    let result = satisfies(&tag_list, Query::Not(
      Box::new(Query::Tag(literal("foo")))
    ));
    assert_eq!(result, false);
  }

  #[test]
  fn tag_list_includes_either_tag() {
    let tag_list = vec![
      literal("foo"),
      literal("bar"),
    ];

    let result = satisfies(&tag_list, Query::Or(
      Box::new(Query::Tag(literal("foo"))),
      Box::new(Query::Tag(literal("bla")))
    ));
    assert_eq!(result, true);
  }

  #[test]
  fn tag_list_does_not_include_either_tag() {
    let tag_list = vec![
      literal("foo"),
      literal("bar"),
    ];

    let result = satisfies(&tag_list, Query::Or(
      Box::new(Query::Tag(literal("pow"))),
      Box::new(Query::Tag(literal("bla")))
    ));
    assert_eq!(result, false);
  }

  #[test]
  fn tag_list_includes_both_tags() {
    let tag_list = vec![
      literal("foo"),
      literal("bar"),
    ];

    let result = satisfies(&tag_list, Query::And(
      Box::new(Query::Tag(literal("foo"))),
      Box::new(Query::Tag(literal("bar")))
    ));
    assert_eq!(result, true);
  }

  #[test]
  fn tag_list_does_not_include_both_tags() {
    let tag_list = vec![
      literal("foo"),
      literal("bar"),
    ];

    let result = satisfies(&tag_list, Query::And(
      Box::new(Query::Tag(literal("foo"))),
      Box::new(Query::Tag(literal("bla")))
    ));
    assert_eq!(result, false);
  }
}
