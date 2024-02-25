use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorKind {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Binary {
        kind: OperatorKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        kind: OperatorKind,
        operand: Box<Expr>,
    },
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Self::Number(value) => *value,
            Self::Binary { kind, left, right } => match kind {
                OperatorKind::Add => left.eval() + right.eval(),
                OperatorKind::Sub => left.eval() - right.eval(),
                OperatorKind::Mul => left.eval() * right.eval(),
                OperatorKind::Div => left.eval() / right.eval(),
            },
            Self::Unary { kind, operand } => match kind {
                OperatorKind::Sub => -operand.eval(),
                _ => panic!("What are you doing? Unknow operator!! {:?}", kind),
            },
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Binary { kind, left, right } => write!(f, "({} {} {})", left, kind, right),
            Expr::Unary { kind, operand } => write!(f, "({} {})", kind, operand),
        }
    }
}

impl Display for OperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorKind::Add => write!(f, "+"),
            OperatorKind::Sub => write!(f, "-"),
            OperatorKind::Mul => write!(f, "*"),
            OperatorKind::Div => write!(f, "/"),
        }
    }
}
