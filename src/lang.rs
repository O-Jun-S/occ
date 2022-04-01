pub mod ast;
pub mod parser;

/// Evaluate expressions.
pub fn eval_expr(s: &str) {
    parser::expr_parser(s)
        .map(|(_, expr)| expr.gen()).unwrap();
}
