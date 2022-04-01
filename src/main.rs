use std::env;
pub mod lang;

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        eprint!("Usage: occ <code>");
        return;
    }

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    lang::gen_expr(&args.nth(1).unwrap());
    print!("  pop rax\n");
    print!("  ret\n");
    return;
}
