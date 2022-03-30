use std::env;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("Usage: occ <code>");
        return;
    }

    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    print!("  mov rax, {}\n", args.nth(1).unwrap());
    print!("  ret\n");
    return;
}

