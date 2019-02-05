use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Mul;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::ops::AddAssign;
use crate::ast::literal::Literal::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct JSLiteral<T> {
    pub value: T
}

pub struct Number(pub i64);

#[derive(Clone, PartialOrd, PartialEq)]
pub enum Literal {
    StringLiteral(String),
    NumericLiteral(i64),
    BooleanLiteral(bool),
    NullLiteral,
    Infinity,
}

impl Add<Literal> for Literal {
    type Output = Literal;
    fn add(self, other: Literal) -> Self::Output {
        match (self, other) {
            (StringLiteral(a), StringLiteral(b)) => StringLiteral(format!("{},{}", a, b)),
            (NumericLiteral(a), StringLiteral(b)) => StringLiteral(format!("{},{}", a, b)),
            (StringLiteral(a), NumericLiteral(b)) => StringLiteral(format!("{},{}", a, b)),
            (StringLiteral(a), NullLiteral) => StringLiteral(a),
            (NullLiteral, StringLiteral(b)) => StringLiteral(b),
            (NumericLiteral(a), NumericLiteral(b)) => NumericLiteral(a + b),
            (Infinity, _) => Infinity,
            (_, Infinity) => Infinity,
            (NullLiteral, NullLiteral) => NullLiteral,
            _ => NullLiteral
        }
    }
}

// Our Js implementation allow you to remove a substring with the minus operator
impl Sub<Literal> for Literal {
    type Output = Literal;
    fn sub(self, other: Literal) -> Self::Output {
        match (self, other) {
            (StringLiteral(a), StringLiteral(b)) => {
                let result = a.replace(b.as_str(), "");
                StringLiteral(result)
            }
            (NumericLiteral(..), StringLiteral(..)) => {
                panic!("Are you trying to substract a string from a numeric literal your silly!?")
            }
            // more silly semantics !!
            (StringLiteral(a), NumericLiteral(b)) => {
                let mut copy = a.clone();
                copy.remove(b as usize);
                StringLiteral(copy)
            }
            (StringLiteral(a), NullLiteral) => StringLiteral(a),
            (NullLiteral, StringLiteral(b)) => StringLiteral(b),
            (NumericLiteral(a), NumericLiteral(b)) => NumericLiteral(a - b),
            (Infinity, _) => Infinity,
            (_, Infinity) => Infinity,
            (NullLiteral, NullLiteral) => NullLiteral,
            _ => NullLiteral
        }
    }
}


impl Mul<Literal> for Literal {
    type Output = Literal;
    fn mul(self, other: Literal) -> Self::Output {
        match (self, other) {
            // "abc" * "123" would return "a123b123c123"
            (StringLiteral(a), StringLiteral(b)) => {
                let mut result: String = String::new();
                a.chars().for_each(|a_char| {
                    result.push(a_char);
                    b.chars().for_each(|b_char| result.push(b_char));
                });
                StringLiteral(result)
            }

            (NumericLiteral(a), StringLiteral(b)) => {
                let mut result: String = String::new();
                for i in 0..a {
                    result.push_str(b.as_str());
                }
                StringLiteral(result)
            }
            (StringLiteral(a), NumericLiteral(b)) => {
                let mut result: String = String::new();
                for i in 0..b {
                    result.push_str(a.as_str());
                }
                StringLiteral(result)
            }
            (StringLiteral(..), NullLiteral) => NullLiteral,
            (NullLiteral, StringLiteral(..)) => NullLiteral,
            (NumericLiteral(a), NumericLiteral(b)) => NumericLiteral(a * b),
            (Infinity, _) => Infinity,
            (_, Infinity) => Infinity,
            (NullLiteral, NullLiteral) => NullLiteral,
            _ => NullLiteral
        }
    }
}

impl Div<Literal> for Literal {
    type Output = Literal;
    fn div(self, other: Literal) -> Self::Output {
        match (self, other) {
            // "abc" * "123" would return "a123b123c123"
            (StringLiteral(a), StringLiteral(b)) => {
                let mut result: String = String::new();
                a.chars().for_each(|a_char| {
                    result.push(a_char);
                    b.chars().for_each(|b_char| result.push(b_char));
                });
                StringLiteral(result)
            }

            (NumericLiteral(a), StringLiteral(b)) => {
                let mut result: String = String::new();
                for i in 0..a {
                    result.push_str(b.as_str());
                }
                StringLiteral(result)
            }
            (StringLiteral(a), NumericLiteral(b)) => {
                let mut result: String = String::new();
                for i in 0..b {
                    result.push_str(a.as_str());
                }
                StringLiteral(result)
            }
            (StringLiteral(..), NullLiteral) => NullLiteral,
            (NullLiteral, StringLiteral(..)) => NullLiteral,
            (NumericLiteral(a), NumericLiteral(b)) => {
                if b == 0 {
                    Infinity
                } else {
                    NumericLiteral(a / b)
                }
            }
            (Infinity, _) => Infinity,
            (_, Infinity) => Infinity,
            (NullLiteral, NullLiteral) => NullLiteral,
            _ => NullLiteral
        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, other: Number) {
        self.0 += other.0
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

impl From<String> for Literal {
    fn from(string: String) -> Self {
        StringLiteral(string)
    }
}

impl From<i64> for Literal {
    fn from(number: i64) -> Self {
        NumericLiteral(number)
    }
}

impl From<bool> for Literal {
    fn from(boolean: bool) -> Self {
        BooleanLiteral(boolean)
    }
}

impl Literal {
    pub fn as_bool(&self) -> bool {
        match self {
            NumericLiteral(num) => {
                if *num != 0 {
                    true
                } else {
                    false
                }
            }
            StringLiteral(..) => true,
            BooleanLiteral(boolean) => *boolean,
            NullLiteral => false,
            Infinity => true,
        }
    }
}


