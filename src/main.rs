use std::env;
pub mod lang;

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        eprint!("Usage: occ <code>");
        return;
    }

    let num = lang::ast::ConstInt::new(
        args.nth(1).unwrap().parse().unwrap(),
    );

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    print!("  mov rax, {}\n", num.get());
    print!("  ret\n");
    return;
}
