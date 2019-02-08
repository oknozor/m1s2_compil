use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Mul;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;
use std::cmp::Ordering;
use std::ops::Rem;
use crate::token::literal::Literal::*;

pub struct Number(pub i64);

#[derive(Clone)]
pub enum Literal {
    StringLiteral(String),
    NumericLiteral(i64),
    BooleanLiteral(bool),
    NullLiteral,
    Infinity,
}

impl Add<Literal> for Literal {
    type Output = Literal;
    fn add(self, rhs: Literal) -> Self::Output {
        match (self, rhs) {
            (StringLiteral(a), StringLiteral(b)) => StringLiteral(format!("{}{}", a, b)),
            (NumericLiteral(a), StringLiteral(b)) => StringLiteral(format!("{}{}", a, b)),
            (StringLiteral(a), NumericLiteral(b)) => StringLiteral(format!("{}{}", a, b)),
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

impl Sub<Literal> for Literal {
    type Output = Literal;
    fn sub(self, rhs: Literal) -> Self::Output {
        match (self, rhs) {
            (StringLiteral(a), StringLiteral(b)) =>
                {
                    let result = a.replace(b.as_str(), "");
                    StringLiteral(result)
                }
            (NumericLiteral(..), StringLiteral(..)) =>
                {
                    panic!("Are you trying to substract a string from a numeric literal your silly!?")
                }
            (StringLiteral(a), NumericLiteral(b)) =>
                {
                    let mut copy = a.clone();
                    copy.remove(b as usize);
                    StringLiteral(copy)
                }
            (NumericLiteral(a), NumericLiteral(b)) => NumericLiteral(a - b),
            (Infinity, _) => Infinity,
            (_, Infinity) => Infinity,
            (StringLiteral(a), NullLiteral) => StringLiteral(a),
            (NullLiteral, StringLiteral(b)) => StringLiteral(b),
            (NullLiteral, NullLiteral) => NullLiteral,
            _ => NullLiteral
        }
    }
}


impl Mul<Literal> for Literal {
    type Output = Literal;
    fn mul(self, rhs: Literal) -> Self::Output {
        match (self, rhs) {
            (StringLiteral(a), StringLiteral(b)) =>
                {
                    let mut result: String = String::new();
                    a.chars().for_each(|a_char| {
                        result.push(a_char);
                        b.chars().for_each(|b_char| result.push(b_char));
                    });
                    StringLiteral(result)
                }
            (NumericLiteral(a), StringLiteral(b)) =>
                {
                    let mut result: String = String::new();
                    for i in 0..a {
                        result.push_str(b.as_str());
                    }
                    StringLiteral(result)
                }
            (StringLiteral(a), NumericLiteral(b)) =>
                {
                    let mut result: String = String::new();
                    for i in 0..b {
                        result.push_str(a.as_str());
                    }
                    StringLiteral(result)
                }

            (NumericLiteral(a), NumericLiteral(b)) => NumericLiteral(a * b),
            (Infinity, _) => Infinity,
            (_, Infinity) => Infinity,
            (StringLiteral(..), NullLiteral) => NullLiteral,
            (NullLiteral, StringLiteral(..)) => NullLiteral,
            (NullLiteral, NullLiteral) => NullLiteral,
            _ => NullLiteral
        }
    }
}

impl Div<Literal> for Literal {
    type Output = Literal;
    fn div(self, rhs: Literal) -> Self::Output {
        match (self, rhs) {
            (StringLiteral(a), StringLiteral(b)) =>
                {
                    let mut result: String = String::new();
                    a.chars().for_each(|a_char| {
                        result.push(a_char);
                        b.chars().for_each(|b_char| result.push(b_char));
                    });
                    StringLiteral(result)
                }

            (NumericLiteral(a), StringLiteral(b)) =>
                {
                    let mut result: String = String::new();
                    for i in 0..a {
                        result.push_str(b.as_str());
                    }
                    StringLiteral(result)
                }

            (StringLiteral(a), NumericLiteral(b)) =>
                {
                    let mut result: String = String::new();
                    for i in 0..b {
                        result.push_str(a.as_str());
                    }
                    StringLiteral(result)
                }
            (NumericLiteral(a), NumericLiteral(b)) =>
                {
                    if b == 0 {
                        Infinity
                    } else {
                        NumericLiteral(a / b)
                    }
                }
            (StringLiteral(..), NullLiteral) => NullLiteral,
            (NullLiteral, StringLiteral(..)) => NullLiteral,
            (NullLiteral, NullLiteral) => NullLiteral,
            (Infinity, _) => Infinity,
            (_, Infinity) => Infinity,
            _ => NullLiteral
        }
    }
}

impl Rem<Literal> for Literal {
    type Output = Literal;
    fn rem(self, rhs: Literal) -> Self::Output {
        match (self, rhs) {
            (NumericLiteral(a), NumericLiteral(b)) => NumericLiteral(a % b),
            _ => panic!("Rem not implemented")
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (StringLiteral(a), StringLiteral(b)) => a.eq(b),
            (NumericLiteral(a), StringLiteral(b)) => format!("{}", a).eq(b),
            (StringLiteral(a), NumericLiteral(b)) => format!("{}", b).eq(a),
            (StringLiteral(a), NullLiteral) => a.eq(&NullLiteral.to_string()),
            (NullLiteral, StringLiteral(b)) => b.eq(&NullLiteral.to_string()),
            (NumericLiteral(a), NumericLiteral(b)) => a == b,
            (Infinity, Infinity) => true,
            (Infinity, StringLiteral(b)) => b.eq(&Infinity.to_string()),
            (StringLiteral(a), Infinity) => a.eq(&Infinity.to_string()),
            (NullLiteral, NullLiteral) => true,
            _ => false
        }
    }

    fn ne(&self, other: &Literal) -> bool { !self.eq(other) }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
        match (self, other) {
            (StringLiteral(a), StringLiteral(b)) => a.partial_cmp(b),
            (NumericLiteral(a), NumericLiteral(b)) => a.partial_cmp(b),
            (NumericLiteral(a), StringLiteral(b)) => format!("{}", a).partial_cmp(b),
            (StringLiteral(a), NumericLiteral(b)) => format!("{}", b).partial_cmp(a),
            (NullLiteral, StringLiteral(b)) => b.partial_cmp(&NullLiteral.to_string()),
            (NullLiteral, NullLiteral) => Some(Ordering::Equal),
            (NullLiteral, _) => Some(Ordering::Less),
            (_, NullLiteral) => Some(Ordering::Greater),
            (Infinity, Infinity) => Some(Ordering::Equal),
            (Infinity, _) => Some(Ordering::Greater),
            (_, Infinity) => Some(Ordering::Less),
            _ => None
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

#[test]
fn should_add_numeric() {}

#[test]
fn should_add() {
    //Num + num
    let a = Literal::NumericLiteral(1);
    let b = Literal::NumericLiteral(1);
    assert_eq!(a + b, Literal::NumericLiteral(2));

    // String + String
    let a = Literal::StringLiteral(String::from("fizz"));
    let b = Literal::StringLiteral(String::from("buzz"));
    let result = a + b;
    assert_eq!(result.to_string(), "fizzbuzz");

    // Num + String
    let a = Literal::NumericLiteral(2);
    let b = Literal::StringLiteral(String::from("fizz"));
    let result = a + b;
    assert_eq!(result.to_string(), "2fizz");

    // String + Num
    let a = Literal::StringLiteral(String::from("fizz"));
    let b = Literal::NumericLiteral(2);
    let result = a + b;
    assert_eq!(result.to_string(), "fizz2");

    // Infinity + Any
    let a = Literal::Infinity;
    let b = Literal::StringLiteral(String::from("fizz"));
    let result = a + b;
    assert_eq!(result.to_string(), "Infinity");

    // Any + Infinity
    let a = Literal::NumericLiteral(3);
    let b = Literal::Infinity;
    let result = a + b;
    assert_eq!(result.to_string(), "Infinity");
}




