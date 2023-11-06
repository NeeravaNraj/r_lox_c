use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub enum Literal {
    None,
    Float(f64),
    Int(isize),
    Bool(bool),
    String(String),
    Variable(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(v) => write!(f, "{v}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Bool(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v}"),
            Self::Variable(v) => write!(f, "{v}"),
            Self::None => write!(f, "none"),
        }
    }
}

impl Literal {
    pub fn negate(self) -> Self {
        match self {
            Self::Float(v) => Self::Float(-v),
            Self::Int(v) => Self::Int(-v),
            _ => panic!("Cannot negate non-number."),
        }
    }

    pub fn truthy(&self) -> bool {
        match self {
            Self::Float(v) => *v != 0.,
            Self::Int(v) => *v != 0,
            Self::Bool(v) => *v,
            Self::String(v) => v.len() > 0,
            Self::None => false,
            _ => unreachable!("variable ?")
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            Self::Float(_) => "float",
            Self::Int(_) => "int",
            Self::Bool(_) => "bool",
            Self::String(_) => "string",
            Self::Variable(_) => "identifier",
            Self::None => "none",
        }
        .to_string()
    }

    pub fn is_number(&self) -> bool {
        match self {
            Self::Float(_) => true,
            Self::Int(_) => true,
            _ => false,
        }
    }

    pub fn not(self) -> Self {
        match self {
            Self::Int(v) => Self::Bool(v == 0),
            Self::Float(v) => Self::Bool(v == 0.),
            Self::None => Self::Bool(true),
            Self::Bool(v) => Self::Bool(!v),
            Self::String(v) => Self::Bool(v.len() == 0),
            _ => self
        }
    }

    pub fn equatable(&self, rhs: &Self) -> Result<(), String> {
        match (self, rhs) {
            (Self::Int(_), Self::Int(_)) |
            (Self::Float(_), Self::Float(_)) |
            (Self::Float(_), Self::Int(_)) |
            (Self::Int(_), Self::Float(_)) |
            (Self::Bool(_), Self::Bool(_)) |
            (Self::Bool(_), Self::Int(_)) |
            (Self::Int(_), Self::Bool(_)) |
            (Self::None, Self::None) |
            (Self::String(_), Self::String(_)) |
            (Self::None, _) |
            (_, Self::None) => return Ok(()),
            _ => Err(format!(
                "Cannot equate types {} and {}",
                self.type_name(),
                rhs.type_name()
            )),
        }
    }

    pub fn comparable(&self, rhs: &Self) -> Result<(), String> {
        match (self, rhs) {
            (Self::Int(_), Self::Int(_)) |
            (Self::Float(_), Self::Float(_)) |
            (Self::Float(_), Self::Int(_)) |
            (Self::Int(_), Self::Float(_)) |
            (Self::Bool(_), Self::Bool(_)) |
            (Self::Bool(_), Self::Int(_)) |
            (Self::String(_), Self::String(_)) |
            (Self::Int(_), Self::Bool(_)) => return Ok(()),
            _ => Err(format!(
                "Cannot compare types {} and {}",
                self.type_name(),
                rhs.type_name()
            )),
        }    
    }
}

impl Add for Literal {
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a + b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a + b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a + *b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(*a as f64 + b)),
            (Self::String(a), Self::String(b)) => Ok(Self::String(a.to_owned() + b)),
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
        match (&self, &rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a - b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a - b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a - *b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(*a as f64 - b)),
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
        match (&self, &rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a * b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a * b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a * *b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(*a as f64 * b)),
            (Self::String(a), Self::Int(b)) => Ok(Self::String(a.repeat(*b as usize))),
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
        match (&self, &rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a / b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a / b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a / *b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(*a as f64 / b)),
            _ => Err(format!(
                "Cannot multiply types {} and {}",
                self.type_name(),
                rhs.type_name()
            )),
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::Float(a), Self::Int(b)) => *a == *b as f64,
            (Self::Int(a), Self::Float(b)) => *a as f64 == *b,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Bool(a), Self::Int(b)) => *a as isize == *b,
            (Self::Int(a), Self::Bool(b)) => *a == *b as isize,
            (Self::String(a), Self::String(b)) => a.eq(b),
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Literal {
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a < b,
            (Self::Float(a), Self::Float(b)) => a < b,
            (Self::Float(a), Self::Int(b)) => *a < *b as f64,
            (Self::Int(a), Self::Float(b)) => (*a as f64) < *b,
            (Self::Bool(a), Self::Bool(b)) => a < b,
            (Self::Bool(a), Self::Int(b)) => (*a as isize) < *b,
            (Self::Int(a), Self::Bool(b)) => *a < *b as isize,
            (Self::String(a), Self::String(b)) => a.len() < b.len(),
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a <= b,
            (Self::Float(a), Self::Float(b)) => a <= b,
            (Self::Float(a), Self::Int(b)) => *a <= *b as f64,
            (Self::Int(a), Self::Float(b)) => (*a as f64) <= *b,
            (Self::Bool(a), Self::Bool(b)) => a <= b,
            (Self::Bool(a), Self::Int(b)) => (*a as isize) <= *b,
            (Self::Int(a), Self::Bool(b)) => *a <= *b as isize,
            (Self::String(a), Self::String(b)) => a.len() <= b.len(),
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a > b,
            (Self::Float(a), Self::Float(b)) => a > b,
            (Self::Float(a), Self::Int(b)) => *a > *b as f64,
            (Self::Int(a), Self::Float(b)) => (*a as f64) > *b,
            (Self::Bool(a), Self::Bool(b)) => a > b,
            (Self::Bool(a), Self::Int(b)) => (*a as isize) > *b,
            (Self::String(a), Self::String(b)) => a.len() > b.len(),
            (Self::Int(a), Self::Bool(b)) => *a > *b as isize,
            _ => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a >= b,
            (Self::Float(a), Self::Float(b)) => a >= b,
            (Self::Float(a), Self::Int(b)) => *a >= *b as f64,
            (Self::Int(a), Self::Float(b)) => (*a as f64) >= *b,
            (Self::Bool(a), Self::Bool(b)) => a >= b,
            (Self::Bool(a), Self::Int(b)) => (*a as isize) >= *b,
            (Self::Int(a), Self::Bool(b)) => *a >= *b as isize,
            (Self::String(a), Self::String(b)) => a.len() >= b.len(),
            _ => false,
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(_), Self::Int(_)) |
            (Self::Float(_), Self::Float(_)) |
            (Self::Float(_), Self::Int(_)) |
            (Self::Int(_), Self::Float(_)) |
            (Self::Bool(_), Self::Bool(_)) |
            (Self::Bool(_), Self::Int(_)) |
            (Self::Int(_), Self::Bool(_)) |
            (Self::String(_), Self::String(_)) => {
                if self < other {
                    return Some(std::cmp::Ordering::Less);
                } else if self == other {
                    return Some(std::cmp::Ordering::Equal);
                } else {
                    return Some(std::cmp::Ordering::Greater);
                } 
            }
            _ => None,
        }
    }
}
