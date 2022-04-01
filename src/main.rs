// use std::env;
pub mod lang;

fn main() {
    // let mut args = env::args();

    // if args.len() != 2 {
    //     eprint!("Usage: occ <code>");
    //     return;
    // }

    // 2 + (3 * 3);
    let node = lang::ast::BinaryOp::new(
        lang::ast::OpKind::Add,
        lang::ast::Expr::ConstInt(lang::ast::ConstInt::new(2)),
        lang::ast::Expr::BinaryOp(
            Box::new(
                lang::ast::BinaryOp::new(
                    lang::ast::OpKind::Mul,
                    lang::ast::Expr::ConstInt(lang::ast::ConstInt::new(3)),
                    lang::ast::Expr::ConstInt(lang::ast::ConstInt::new(3)),
                )
            )
        )
    );

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    node.gen();
    print!("  pop rax\n");
    print!("  ret\n");
    return;
}
