use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    Float(f64),
    Int(i32),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(v) => write!(f, "{v}"),
            Literal::Int(v) => write!(f, "{v}"),
        }
    }
}

impl Literal {
    pub fn negate(&self) -> Self {
        match self {
            Self::Float(v) => Self::Float(-v),
            Self::Int(v) => Self::Int(-v),
            _ => panic!("Cannot negate non-number."),
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            Self::Float(_) => "float",
            Self::Int(_) => "int",
        }
        .to_string()
    }
}

impl Add for Literal {
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a + b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a + b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a + b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 + b)),
            _ => Err(format!(
                "Cannot add types {} and {}",
                self.type_name(),
                rhs.type_name()
            )),
        }
    }
}

impl Sub for Literal {
    type Output = Result<Self, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a - b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a - b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a - b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 - b)),
            _ => Err(format!(
                "Cannot subtract types {} and {}",
                self.type_name(),
                rhs.type_name()
            )),
        }
    }
}

impl Mul for Literal {
    type Output = Result<Self, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a * b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a * b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a * b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 * b)),
            _ => Err(format!(
                "Cannot multiply types {} and {}",
                self.type_name(),
                rhs.type_name()
            )),
        }
    }
}

impl Div for Literal {
    type Output = Result<Self, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a / b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a / b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a / b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 / b)),
            _ => Err(format!(
                "Cannot multiply types {} and {}",
                self.type_name(),
                rhs.type_name()
            )),
        }
    }
}
