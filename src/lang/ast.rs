/// It represents constant integer.
pub struct ConstInt(i32);


impl ConstInt {
    /// Create an constint.
    pub fn new(n: i32) -> ConstInt {
        ConstInt(n)
    }

    /// Generate assembly.
    pub fn gen(&self) {
        print!("  push {}\n", self.0);
    }
}

/// Represents expression.
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

/// Kinds of operators.
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}

impl Expr {
    pub fn gen(&self) {
        match self {
            Expr::ConstInt(expr) => expr.gen(),
            Expr::BinaryOp(expr) => expr.gen(),
        }
    }
}

impl BinaryOp {
    /// Create a new BinaryOp.
    pub fn new(op_kind: OpKind, lhs: Expr, rhs: Expr) -> BinaryOp {
        BinaryOp { op_kind, lhs, rhs }
    }

    /// Generate assembly.
    pub fn gen(&self) {
        self.lhs.gen();
        self.rhs.gen();
        print!("  pop rdi\n");
        print!("  pop rax\n");
        match &self.op_kind {
            OpKind::Add => print!("  add rax, rdi\n"),
            OpKind::Sub => print!("  sub rax, rdi\n"),
            OpKind::Mul => print!("  imul rax, rdi\n"),
            OpKind::Div => print!("  cqo\n  idiv rdi\n"),
        };
        print!("  push rax\n");
    }
}
