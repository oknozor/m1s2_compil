use crate::token::token::BinaryOperator;
use crate::token::token::BinaryOperator::*;
use crate::token::token::LogicalOperator;
use crate::token::token::LogicalOperator::*;
use crate::token::token::AssignmentOperator;
use crate::token::token::AssignmentOperator::*;
use crate::token::token::UpdateOperator;
use crate::token::token::UpdateOperator::*;
use crate::token::token::UnaryOperator;
use crate::token::token::UnaryOperator::*;
use crate::token::token::Operator;
use crate::token::token::Operator::*;
use crate::token::token::Literal;
use crate::token::token::Literal::*;
use crate::token::token::Token;
use crate::token::token::Token::*;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;
use crate::token::token::Call;

pub mod token;
pub mod op_overload;
pub mod to_token;

impl Operator {
    pub fn as_str(&self) -> &str {
        match self {
            UnaryOp(o) => o.as_str(),
            LogOp(o) => o.as_str(),
            UpdateOp(o) => o.as_str(),
            BinOp(o) => o.as_str(),
            AssignOp(o) => o.as_str(),
            LeftParenthesis => "(",
            RightParenthesis => ")",
        }
    }
}

impl Literal {
    pub fn to_string(self) -> String {
        match self {
            NullLiteral => "null".to_string(),
            Infinity => "Infinity".to_string(),
            NumericLiteral(n) => format!("{}", n),
            StringLiteral(s) => s,
            BooleanLiteral(b) => if b { "1".to_string() } else { "0".to_string() }
        }
    }
}

pub trait Precedence {
    fn get_precedence(a: &Self, b: &Self) -> bool;
    fn is_left_associative(&self) -> bool;
}

impl Precedence for BinaryOperator {
    fn get_precedence(a: &BinaryOperator, b: &BinaryOperator) -> bool {
        match (a, b) {
            (Mul, _) => true,
            (Div, _) => true,
            (Mod, _) => true,
            _ => false,
        }
    }

    fn is_left_associative(&self) -> bool {
        match self {
            Div => true,
            Sub => true,
            _ => false,
        }
    }
}

impl Precedence for Operator {
    fn get_precedence(a: &Self, b: &Self) -> bool {
        match (a, b) {
            (UnaryOp(_), BinOp(_)) => true,
            (BinOp(a), BinOp(b)) => BinaryOperator::get_precedence(a, b),
            (BinOp(_), _) => true,
            (AssignOp(_), _) => false,
            _ => false,
        }
    }

    fn is_left_associative(&self) -> bool {
        match self {
            BinOp(op) => op.is_left_associative(),
            _ => false
        }
    }
}

impl From<&str> for BinaryOperator {
    fn from(str_op: &str) -> Self {
        match str_op {
            "+" => Add,
            "-" => Sub,
            "*" => Mul,
            "/" => Div,
            "%" => Mod,
            "<" => LessThan,
            "<=" => LessThanOrEq,
            ">" => GreaterThan,
            ">=" => GreaterThanOrEq,
            "!=" => PartialEq,
            "==" => StrictEq,
            _ => panic!("Unkown token")
        }
    }
}

impl BinaryOperator {
    pub fn as_str<'op>(&self) -> &'op str {
        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            LessThan => "<",
            LessThanOrEq => "<=",
            GreaterThan => ">",
            GreaterThanOrEq => ">=",
            PartialEq => "!=",
            StrictEq => "==",
        }
    }
}


impl From<&str> for LogicalOperator {
    fn from(str_op: &str) -> Self {
        match str_op {
            "||" => Or,
            "&&" => And,
            _ => panic!("Unkown token")
        }
    }
}

impl LogicalOperator {
    pub fn as_str<'op>(&self) -> &'op str {
        match self {
            Or => "||",
            And => "&&",
            _ => "NoP"
        }
    }
}

impl From<&str> for AssignmentOperator {
    fn from(str_op: &str) -> Self {
        match str_op {
            "+=" => AddAssign,
            "-=" => SubAssign,
            "/=" => DivAssign,
            "*=" => MulAssign,
            "%=" => ModAssign,
            _ => panic!("Unkown token")
        }
    }
}

impl AssignmentOperator {
    pub fn as_str<'op>(&self) -> &'op str {
        match self {
            AddAssign => "+=",
            SubAssign => "-=",
            DivAssign => "/=",
            MulAssign => "*=",
            ModAssign => "%=",
        }
    }
}

impl From<&str> for UpdateOperator {
    fn from(str_op: &str) -> Self {
        match str_op {
            "--" => Decrement,
            "++" => Increment,
            _ => panic!("Unkown token")
        }
    }
}

impl UpdateOperator {
    pub fn as_str<'op>(&self) -> &'op str {
        match self {
            Increment => "++",
            Decrement => "--",
            _ => "NoP"
        }
    }
}

impl From<&str> for UnaryOperator {
    fn from(str_op: &str) -> Self {
        match str_op {
            "+" => Plus,
            "-" => Minus,
            "!" => ExPoint,
            "~" => Tilde,
            "void" => Void,
            "typeof" => TypeOf,
            "delete" => Delete,
            _ => panic!("Unkown token")
        }
    }
}

impl UnaryOperator {
    pub fn as_str<'op>(&self) -> &'op str {
        match self {
            Plus => "+",
            Minus => "-",
            ExPoint => "!",
            Tilde => "~",
            TypeOf => "void",
            Void => "typeof",
            Delete => "delete",
            _ => "NoP"
        }
    }
}


impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self {
            Token::LiteralToken(b) => write!(f, "{}", b),
            Token::OperatorToken(op) => write!(f, "{}", op.as_str()),
            Token::Undefined => write!(f, "Undefined"),
            Token::IdendifierToken(id) => write!(f, "{}", id),
            Token::FunctionToken(Call { args, callee }) => write!(f, "{:?} ({:?})", callee, args),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Token::LiteralToken(literal) => write!(f, "{}", literal),
            Token::OperatorToken(op) => write!(f, "{}", op.as_str()),
            Token::Undefined => write!(f, "{}", "Undefined"),
            Token::IdendifierToken(id) => write!(f, "{}", id),
            Token::FunctionToken(Call { args, callee }) => write!(f, "Call {:?} {:?}", callee, args),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            StringLiteral(string) => write!(f, "{}", string),
            NumericLiteral(num) => write!(f, "{}", num),
            BooleanLiteral(boolean) => write!(f, "{}", boolean),
            NullLiteral => write!(f, "{}", "Null"),
            Infinity => write!(f, "{}", "Infinity"),
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            StringLiteral(string) => write!(f, "{}", string),
            NumericLiteral(num) => write!(f, "{}", num),
            BooleanLiteral(boolean) => write!(f, "{}", boolean),
            NullLiteral => write!(f, "{}", "Null"),
            Infinity => write!(f, "{}", "Infinity"),
        }
    }
}

impl Display for UpdateOperator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}

impl Display for LogicalOperator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}

impl Display for AssignmentOperator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}

impl From<String> for Literal {
    fn from(string: String) -> Self {
        StringLiteral(string)
    }
}

impl From<f64> for Literal {
    fn from(number: f64) -> Self {
        NumericLiteral(number)
    }
}

impl From<bool> for Literal {
    fn from(boolean: bool) -> Self {
        BooleanLiteral(boolean)
    }
}

impl From<Literal> for Token {
    fn from(literal: Literal) -> Self {
        LiteralToken(literal)
    }
}

impl From<Operator> for Token {
    fn from(operator: Operator) -> Self {
        OperatorToken(operator)
    }
}

impl Token {
    pub fn as_operator(&self) -> Result<Operator, Error> {
        match self {
            OperatorToken(op) => Ok(op.clone()),
            _ => panic!("not an operator")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::token::Operator;
    use crate::token::token::BinaryOperator::*;
    use crate::token::Precedence;

    #[test]
    fn mul_shall_have_precedence() {
        let mul = Operator::BinOp(Mul);
        let add = Operator::BinOp(Add);

        let as_precedence = Precedence::get_precedence(&mul, &add);

        assert_eq!(true, as_precedence)
    }
}

