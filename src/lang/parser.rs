use super::ast::*;
use nom::{
    IResult,
    character::complete::digit1,
};


/// primary = constint | "(" expr ")"
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
