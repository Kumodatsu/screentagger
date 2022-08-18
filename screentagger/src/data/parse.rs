use crate::data::tag::{literal};
use crate::data::query::{Query, Query::*};
use nom::{
  IResult,
  Parser,
  branch::alt,
  bytes::complete::tag,
  character::complete::{
    multispace0,
    multispace1,
    alphanumeric1,
  },
  combinator::{
    complete,
    map,
    opt,
    value,
  },
  error::ParseError,
  multi::many0,
  sequence::{
    delimited,
    separated_pair,
    tuple,
  },
};

fn p_query(s: &str) -> IResult<&str, Query> {
  p_or(s)
}

fn p_or(s: &str) -> IResult<&str, Query> {
  let (s, expr)  = p_and(s)?;
  let (s, exprs) = many0(tuple((
    tag("|"),
    p_and,
  )))(s)?;
  println!("{}, {:?}", s, exprs);
  Ok((s, p_op(expr, exprs)))
}

fn p_and(s: &str) -> IResult<&str, Query> {
  let (s, expr)  = p_tag(s)?;
  let (s, exprs) = many0(tuple((
    tag("&"),
    p_tag,
  )))(s)?;
  Ok((s, p_op(expr, exprs)))
}

fn p_tag(s: &str) -> IResult<&str, Query> {
    map(
      delimited(multispace0, alphanumeric1, multispace0),
      |x| Tag(literal(x))
    )(s)
}

fn p_op(q: Query, r: Vec<(&str, Query)>) -> Query {
  r.into_iter().fold(q, |acc, (op, q2)| {
    match op {
      "|" => Or(Box::new(acc), Box::new(q2)),
      "&" => And(Box::new(acc), Box::new(q2)),
      _   => panic!("Unknown operator."),
    }
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::data::{
    query::Query::*,
    tag::Tag::*,
  };
  
  #[test]
  fn tag_is_parsed() {
    let input  = "foo baz bar";
    let result = p_tag(input);
    assert_eq!(
      result,
      Ok(("baz bar", Tag(Literal(String::from("foo")))))
    );
  }

  #[test]
  fn and_is_parsed() {
    let input  = "foo & baz";
    let result = p_and(input);
    assert_eq!(
      result,
      Ok(("", And(
        Box::new(Tag(Literal(String::from("foo")))),
        Box::new(Tag(Literal(String::from("baz"))))
      )))
    );
  }

  #[test]
  fn or_is_parsed() {
    let input  = "foo | baz";
    let result = p_or(input);
    assert_eq!(
      result,
      Ok(("", Or(
        Box::new(Tag(Literal(String::from("foo")))),
        Box::new(Tag(Literal(String::from("baz"))))
      )))
    );
  }

  #[test]
  fn and_after_or_has_correct_presedence() {
    let input  = "foo | baz & bar";
    let result = p_query(input);
    assert_eq!(
      result,
      Ok(("", Or(
        Box::new(Tag(Literal(String::from("foo")))),
        Box::new(And(
          Box::new(Tag(Literal(String::from("baz")))),
          Box::new(Tag(Literal(String::from("bar"))))
        ))
      )))
    );
  }

  #[test]
  fn or_after_and_has_correct_presedence() {
    let input  = "foo & baz | bar";
    let result = p_query(input);
    assert_eq!(
      result,
      Ok(("", Or(
        Box::new(And(
          Box::new(Tag(Literal(String::from("foo")))),
          Box::new(Tag(Literal(String::from("baz"))))
        )),
        Box::new(Tag(Literal(String::from("bar"))))
      )))
    );
  }

}
