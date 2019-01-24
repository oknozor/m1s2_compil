pub enum NumOp {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
}
pub enum UnaryOp {
    IncrementPost,
    IncrementPre,
    DecrementPost,
    DecrementPre,
    Minus,
    Plus,
    Not,
}
pub enum BitOp {
    And,
    Or,
    Xor,
    ShiftL,
    ShiftR,
}

pub enum CompOp {
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

pub enum LogOp {
    And,
    Or,
}
