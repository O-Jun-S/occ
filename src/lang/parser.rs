use super::ast::*;
use nom::{
    IResult,
    character::complete::digit1,
    sequence::delimited,
    bytes::complete::tag,
};


/// primary = constint | "(" expr ")"
/// Parse an integer.
pub fn constint_parser(s: &str) -> IResult<&str, ConstInt> {
    let (no_used, num_str) = digit1(s)?;
    let num: i32 = num_str.parse().expect("Expected number.");
    Ok((no_used, ConstInt::new(num)))
}

#[test]
fn constint_parser_test() {
    let expect = ConstInt::new(33);
    let (_, actual) = constint_parser("33").unwrap();
    assert_eq!(
        actual,
        expect,
    );
}

/// Expression parser(temporary).
pub fn expr_parser(s: &str) -> IResult<&str, Expr> {
    constint_parser(s)
        .map(|(no_used, constint)|
            (no_used, Expr::ConstInt(constint))
        )
}

/// Parse string containing parentheses.
/// "(" expr ")"
pub fn parentheses_parser(s: &str) -> IResult<&str, Expr> {
    delimited(tag("("), expr_parser, tag(")"))(s)
}

#[test]
fn parentheses_parser_test() {
    let expected = Expr::ConstInt(ConstInt::new(64));
    let (_, actual) = parentheses_parser("(64)").unwrap();
    assert_eq!(
        expected,
        actual,
    );
}
