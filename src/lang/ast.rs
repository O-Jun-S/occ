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

pub enum Expr {
    ConstInt(ConstInt),
    BinaryOp(Box<BinaryOp>),
}

// Represents binary operator.
pub struct BinaryOp {
    op_kind: OpKind,
    lhs: Expr,
    rhs: Expr,
}

// Kinds of operators.
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}

impl Expr {
    pub fn gen(&self) -> i32 {
        match self {
            Expr::ConstInt(expr) => expr.gen(),
            Expr::BinaryOp(expr) => expr.gen(),
        }
    }
}

impl BinaryOp {
    pub fn new(op_kind: OpKind, lhs: Expr, rhs: Expr) -> BinaryOp {
        BinaryOp { op_kind, lhs, rhs }
    }

    pub fn gen(&self) -> i32 {
        print!("  pop rdi\n");
        print!("  pop rax\n");
        match &self.op_kind {
            OpKind::Add => print!("  add rax, rdi\n"),
            OpKind::Sub => print!("  sub rax, rdi\n"),
            OpKind::Mul => print!("  imul rax, rdi\n"),
            OpKind::Div => print!("  cqo\n  idiv rdi\n"),
        };
        print!("  push rax");

        match &self.op_kind {
            OpKind::Add => self.lhs.gen() + self.rhs.gen(),
            OpKind::Sub => self.lhs.gen() - self.rhs.gen(),
            OpKind::Mul => self.lhs.gen() * self.rhs.gen(),
            OpKind::Div => self.lhs.gen() / self.rhs.gen(),
        }
    }
}
