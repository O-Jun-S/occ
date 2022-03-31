/// It represents constant integer.
pub struct ConstInt(i32);


impl ConstInt {
    pub fn new(n: i32) -> ConstInt {
        ConstInt(n)
    }

    /// Evaluate an integer.
    pub fn gen(&self) -> i32 {
        print!("  push {}\n", self.0);
        self.0
    }
}

#[test]
fn constint_test() {
    let expect = 128;
    let num = ConstInt::new(expect);
    assert_eq!(num.gen(), expect);
}

pub enum Expr {
    ConstInt(ConstInt),
    OpPlus(Box<OpPlus>),
}

pub struct OpPlus {
    lhs: Expr,
    rhs: Expr,
}

impl Expr {
    pub fn gen(&self) -> i32 {
        match self {
            Expr::ConstInt(expr) => expr.gen(),
            Expr::OpPlus(expr) => expr.gen(),
        }
    }
}

impl OpPlus {
    pub fn new(lhs: Expr, rhs: Expr) -> OpPlus {
        OpPlus { lhs, rhs }
    }

    pub fn gen(&self) -> i32 {
        print!("  pop rdi\n");
        print!("  pop rax\n");
        print!("  add rax, rdi\n");
        self.lhs.gen() + self.rhs.gen()
    }
}

#[test]
fn op_plus_test() {
    let expect = 1 + (2 + 3);
    let num = OpPlus::new(
        Expr::ConstInt(ConstInt::new(1)),
        Expr::OpPlus(
            Box::new(
                OpPlus::new(
                    Expr::ConstInt(ConstInt::new(2)),
                    Expr::ConstInt(ConstInt::new(3)),
                )
            )
        ),
    );

    assert_eq!(
        num.gen(),
        expect,
    );
}
