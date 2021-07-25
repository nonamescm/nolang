use crate::error;
use std::{cmp, fmt, ops};

#[derive(Debug, Clone)]
/// Nolang primitive types
pub enum Primitive {
    Number(f64),
    Str(String),
    Bool(bool),
    None,
}

pub trait IntoPrimitive {
    fn into_pri(self) -> Primitive;
}

impl PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Str(s_str), Self::Str(o_str)) => s_str == o_str,
            (Self::Number(s_num), Self::Number(o_num)) => s_num == o_num,
            (Self::Bool(s_bool), Self::Bool(o_bool)) => s_bool == o_bool,
            (Self::None, Self::None) => true,
            _ => error!("TypeError"; "can't compare {} with {} using == or ~=", self, other => 1),
        }
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let raw = match self {
            Self::Bool(true) => "true".to_string(),
            Self::Bool(false) => "false".to_string(),
            Self::None => "none".to_string(),
            Self::Str(s) => s.to_string(),
            Self::Number(ref n) => n.to_string(),
        };
        write!(f, "{}", raw)
    }
}

impl ops::Not for Primitive {
    type Output = bool;

    fn not(self) -> Self::Output {
        !self.to_bool()
    }
}

impl ops::Neg for Primitive {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(n) => Primitive::Number(-n),
            Self::Bool(true) => Primitive::Number(-1f64),
            Self::Bool(false) => Primitive::Number(0f64),
            Self::Str(s) => Primitive::Number(s.len() as f64),
            Self::None => Primitive::Number(0f64),
        }
    }
}

impl ops::Add for Primitive {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (rhs.to_number(), self.to_number()) {
            (Some(o_num), Some(s_num)) => Self::Number(o_num + s_num),
            _ => error!("TypeError"; "tried to use `+` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Sub for Primitive {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (rhs.to_number(), self.to_number()) {
            (Some(o_num), Some(s_num)) => Self::Number(o_num - s_num),
            _ => error!("TypeError"; "tried to use `-` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Mul for Primitive {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (rhs.to_number(), self.to_number()) {
            (Some(o_num), Some(s_num)) => Self::Number(o_num * s_num),
            _ => error!("TypeError"; "tried to use `*` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Div for Primitive {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (rhs.to_number(), self.to_number()) {
            (Some(o_num), Some(s_num)) => Self::Number(o_num / s_num),
            _ => error!("TypeError"; "tried to use `/` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl cmp::PartialOrd for Primitive {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Self::Number(s_num), Self::Number(o_num)) => o_num.partial_cmp(s_num),
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
            Self::Number(x) if x.abs() < f64::EPSILON => false,
            Self::Str(b) if b.as_str() == "" => false,
            _ => true,
        }
    }

    pub fn to_number(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            Self::Str(..) => None,
            Self::Bool(..) => None,
            Self::None => None,
        }
    }
}

impl IntoPrimitive for bool {
    fn into_pri(self) -> Primitive {
        Primitive::Bool(self)
    }
}

impl IntoPrimitive for String {
    fn into_pri(self) -> Primitive {
        Primitive::Str(self)
    }
}

impl IntoPrimitive for f64 {
    fn into_pri(self) -> Primitive {
        Primitive::Number(self)
    }
}
