use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Mul;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::ops::AddAssign;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Literal<T> {
    pub value: T
}

#[derive(Copy, Clone)]
pub struct Number(pub i64);
impl Add<Number> for Number {
    type Output = Number;
    fn add(self, other: Number) -> Self::Output {
        Number(self.0 + other.0)
    }
}

impl Sub<Number> for Number {
    type Output = Number;
    fn sub(self, other: Number) -> Self::Output {
        Number(self.0 - other.0)
    }
}

impl Mul<Number> for Number {
    type Output = Number;
    fn mul(self, other: Number) -> Self::Output {
        Number(self.0 * other.0)
    }
}
impl Div<Number> for Number {
    type Output = Number;
    fn div(self, other: Number) -> Self::Output {
        Number(self.0 / other.0)
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, other: Number) {
        self.0 += other.0
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

