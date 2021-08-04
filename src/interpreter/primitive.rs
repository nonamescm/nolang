use super::Statement;
use crate::error;
use std::{cmp, fmt, ops};

/// Nolang primitive types
#[derive(Debug, Clone)]
pub enum Primitive {
    Int(i32),
    Float(f64),
    Str(String),
    Bool(bool),
    Function(Statement, Vec<String>),
    NativeFunc(fn(arg: Primitive) -> Primitive),
    None,
}

pub trait IntoPrimitive {
    fn into_pri(self) -> Primitive;
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let raw = match self {
            Self::Bool(true) => "true".to_string(),
            Self::Bool(false) => "false".to_string(),
            Self::None => "none".to_string(),
            Self::Str(s) => s.to_string(),
            Self::Float(ref n) => n.to_string(),
            Self::Int(ref n) => n.to_string(),
            Self::Function(..) => "<function>".to_string(),
            Self::NativeFunc(..) => "<native function>".to_string(),
        };
        write!(f, "{}", raw)
    }
}

impl ops::Not for Primitive {
    type Output = bool;

    #[inline]
    fn not(self) -> Self::Output {
        !self.to_bool()
    }
}

impl ops::Neg for Primitive {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Self::Float(n) => Self::Float(-n),
            Self::Int(n) => Self::Int(-n),
            Self::Bool(true) => Self::Int(-1),
            Self::Bool(false) => Self::Int(0),
            _ => error!("TypeError"; "can't use `-` operator with {}", self => 1),
        }
    }
}

impl ops::Add for Primitive {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        match (&rhs, &self) {
            (Self::Int(o_num), Self::Int(s_num)) => Self::Int(o_num + s_num),
            (Self::Float(o_num), Self::Float(s_num)) => Self::Float(o_num + s_num),
            (Self::Int(o_num), Self::Float(s_num)) => Self::Float(*o_num as f64 + s_num),
            (Self::Float(o_num), Self::Int(s_num)) => Self::Float(o_num + *s_num as f64),

            (Self::Str(o_str), Self::Str(s_str)) => Self::Str(o_str.to_string() + s_str),
            _ => error!("TypeError"; "tried to use `+` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Sub for Primitive {
    type Output = Primitive;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        match (&rhs, &self) {
            (Self::Int(o_num), Self::Int(s_num)) => Self::Int(o_num - s_num),
            (Self::Float(o_num), Self::Float(s_num)) => Self::Float(o_num - s_num),
            (Self::Int(o_num), Self::Float(s_num)) => Self::Float(*o_num as f64 - s_num),
            (Self::Float(o_num), Self::Int(s_num)) => Self::Float(o_num - *s_num as f64),
            _ => error!("TypeError"; "tried to use `-` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Mul for Primitive {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        match (&rhs, &self) {
            (Self::Int(o_num), Self::Int(s_num)) => Self::Int(s_num * o_num),
            (Self::Float(o_num), Self::Float(s_num)) => Self::Float(o_num * s_num),
            (Self::Int(o_num), Self::Float(s_num)) => Self::Float(*o_num as f64 * s_num),
            (Self::Float(o_num), Self::Int(s_num)) => Self::Float(o_num * *s_num as f64),
            _ => error!("TypeError"; "tried to use `*` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Div for Primitive {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        match (&rhs, &self) {
            (Self::Int(o_num), Self::Int(s_num)) => Self::Int(o_num / s_num),
            (Self::Float(o_num), Self::Float(s_num)) => Self::Float(o_num / s_num),
            (Self::Int(o_num), Self::Float(s_num)) => Self::Float(*o_num as f64 / s_num),
            (Self::Float(o_num), Self::Int(s_num)) => Self::Float(o_num / *s_num as f64),
            _ => error!("TypeError"; "tried to use `/` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Rem for Primitive {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        match (&rhs, &self) {
            (Self::Int(o_num), Self::Int(s_num)) => Self::Int(o_num % s_num),


            (Self::Float(o_num), Self::Float(s_num)) => Self::Float(o_num % s_num),
            (Self::Int(o_num), Self::Float(s_num)) => Self::Float(*o_num as f64 % s_num),
            (Self::Float(o_num), Self::Int(s_num)) => Self::Float(o_num % *s_num as f64),
            _ => error!("TypeError"; "tried to use `%` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl cmp::PartialEq for Primitive {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(o_num), Self::Int(s_num)) => o_num == s_num,


            (Self::Float(o_num), Self::Float(s_num)) => o_num == s_num,
            (Self::Int(o_num), Self::Float(s_num)) => *o_num as f64 == *s_num,
            (Self::Float(o_num), Self::Int(s_num)) => *o_num == *s_num as f64,

            (Self::Bool(s_bool), Self::Bool(o_bool)) => s_bool == o_bool,
            (Self::None, Self::None) => true,
            _ => error!("TypeError"; "can't compare {} with {} using == or ~=", self, other => 1),
        }
    }
}

impl cmp::PartialOrd for Primitive {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Self::Int(s_num), Self::Int(o_num)) => o_num.partial_cmp(s_num),
            (Self::Float(s_num), Self::Float(o_num)) => o_num.partial_cmp(s_num),
            (Self::Str(s_str), Self::Str(o_str)) => o_str.partial_cmp(s_str),
            (Self::Bool(s_bool), Self::Bool(o_bool)) => o_bool.partial_cmp(s_bool),
            _ => {
                error!("TypeError"; "can't compare {} with {} using <, >, <=, >=", self, other => 1)
            }
        }
    }
}

impl Primitive {
    pub fn to_bool(&self) -> bool {
        match self {
            Self::Bool(false) => false,
            Self::None => false,
            Self::Float(x) if x.abs() < f64::EPSILON => false,
            Self::Int(0) => false,
            Self::Str(b) if b.as_str() == "" => false,
            _ => true,
        }
    }

    pub fn to_number(&self) -> Option<Self> {
        match self {
            Self::Float(n) => Some(Self::Float(*n)),
            Self::Int(n) => Some(Self::Int(*n)),
            _ => None,
        }
    }

    pub fn and(self, rhs: &mut dyn FnMut() -> Self) -> Self {
        match self.to_bool() {
            true => rhs(),
            false => self,
        }
    }

    pub fn or(self, rhs: &mut dyn FnMut() -> Self) -> Self {
        match self.to_bool() {
            true => self,
            false => rhs(),
        }
    }

    pub fn pow(&self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Self::Int(s_num), Self::Int(o_num)) => Primitive::Int(s_num.pow(*o_num as u32)),
            (Self::Float(s_num), Self::Int(o_num)) => Primitive::Float(s_num.powf(*o_num as f64)),
            (Self::Int(s_num), Self::Float(o_num)) => Primitive::Float(o_num.powf(*s_num as f64)),
            (Self::Float(s_num), Self::Float(o_num)) => Primitive::Float(s_num.powf(*o_num)),
            _ => crate::error!("RuntimeError"; "Can't apply operator `**` between {} and {}", &self, &rhs => 1)
        }
    }
}

impl IntoPrimitive for bool {
    #[inline]
    fn into_pri(self) -> Primitive {
        Primitive::Bool(self)
    }
}

impl IntoPrimitive for String {
    #[inline]
    fn into_pri(self) -> Primitive {
        Primitive::Str(self)
    }
}

impl IntoPrimitive for f64 {
    #[inline]
    fn into_pri(self) -> Primitive {
        Primitive::Float(self)
    }
}

impl IntoPrimitive for i32 {
    #[inline]
    fn into_pri(self) -> Primitive {
        Primitive::Int(self)
    }
}
