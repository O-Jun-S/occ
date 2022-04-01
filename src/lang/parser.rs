use super::ast::*;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    combinator::opt,
    sequence::tuple,
    sequence::delimited,
    character::complete::digit1,
    character::complete::char,
};


/// <factor> := <const_int> | <paren_expr>
pub fn factor_parser(s: &str) -> IResult<&str, Expr> {
    alt((
        map(
            constint_parser,
            |constint| Expr::ConstInt(constint),
        ),
        parentheses_parser,
    ))(s)
}

#[test]
fn factor_parser_test() {
    let expected1 = Expr::ConstInt(ConstInt::new(5));
    let (_, actual1) = factor_parser("5").unwrap();
    assert_eq!(
        expected1,
        actual1,
    );

    let expected2 = Expr::ConstInt(ConstInt::new(10));
    let (_, actual2) = factor_parser("(10)").unwrap();
    assert_eq!(
        expected2,
        actual2,
    );
}

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
/// expr = mul ("+" mul | "-" mul)*
pub fn expr_parser(s: &str) -> IResult<&str, Expr> {
    let op_kind_parser =
        map(
            alt((char('+'), char('-'))),
            |op_char|
                match op_char {
                    '+' => OpKind::Add,
                    '-' => OpKind::Sub,
                    _ => panic!("Expected + or - !")
                },
        );
    
    let binaryop_parser = tuple((
        term_parser,
        opt(
            tuple((
                op_kind_parser,
                expr_parser,
            ))
        )
    ));

    map(binaryop_parser, |(lhs, rhs_opt)| {
        if let Option::Some((op_kind, rhs)) = rhs_opt {
            Expr::BinaryOp(
                Box::new(
                    BinaryOp::new(op_kind, lhs, rhs),
                )
            )
        } else {
            lhs
        }
    })(s)
}

/// Parse string containing parentheses.
/// <paren_expr> := "(" <expr> ")"
pub fn parentheses_parser(s: &str) -> IResult<&str, Expr> {
    let res_opt = opt(
        delimited(
            tag("("),
            expr_parser,
            tag(")")
        ),
    )(s).unwrap();

    if let (no_used, Option::Some(expr)) = res_opt {
        Ok((no_used, expr))
    }

    else {
        expr_parser(s)
    }
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

/// <term> := <factor> [ ('*'|'/') <term> ]
/// Parsing expressions multiplying and dividing.
pub fn term_parser(s: &str) -> IResult<&str, Expr> {
    // Parse * and /
    let op_kind_parser = map(
        alt((char('*'), char('/'))),
        |op_char|
                match op_char {
                    '*' => OpKind::Mul,
                    '/' => OpKind::Div,
                    _ => panic!("Expected * or /"),
                },
    );

    let binaryop_parser = tuple((
        factor_parser,
        opt(
            tuple((
                op_kind_parser,
                term_parser,
            ))
        )
    ));

    map(binaryop_parser, |(lhs, rhs_opt)| {
        if let Option::Some((op_kind, rhs)) = rhs_opt {
            Expr::BinaryOp(
                Box::new(
                    BinaryOp::new(
                        op_kind,
                        lhs,
                        rhs,
                    )
                )
            )
        }

        else {
            lhs
        }
    })(s)
}

#[test]
fn term_parser_test() {
    let (_, actual) = term_parser("2*3/3").unwrap();

    let dividing = Expr::BinaryOp(
        Box::new(
            BinaryOp::new(
                OpKind::Div,
                Expr::ConstInt(ConstInt::new(3)),
                Expr::ConstInt(ConstInt::new(3)),
            )
        )
    );

    let expected = Expr::BinaryOp(
        Box::new(
            BinaryOp::new(
                OpKind::Mul,
                Expr::ConstInt(ConstInt::new(2)),
                dividing,
            )
        )
    );

    assert_eq!(expected, actual);
}
