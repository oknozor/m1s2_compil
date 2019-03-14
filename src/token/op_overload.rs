use std::ops::Add;
use crate::token::token::Literal;
use crate::token::token::Literal::*;
use std::ops::Sub;
use std::ops::Div;
use std::ops::Rem;
use std::cmp::Ordering;
use std::ops::Mul;

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
                    for i in 0..a as i64 {
                        result.push_str(b.as_str());
                    }
                    StringLiteral(result)
                }
            (StringLiteral(a), NumericLiteral(b)) =>
                {
                    let mut result: String = String::new();
                    for i in 0..b as i64 {
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
                    for i in 0..a as i64 {
                        result.push_str(b.as_str());
                    }
                    StringLiteral(result)
                }

            (StringLiteral(a), NumericLiteral(b)) =>
                {
                    let mut result: String = String::new();
                    for i in 0..b as i64 {
                        result.push_str(a.as_str());
                    }
                    StringLiteral(result)
                }
            (NumericLiteral(a), NumericLiteral(b)) =>
                {
                    if b as i64 == 0 {
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