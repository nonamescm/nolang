use super::Statement;
use crate::error;
use std::{cmp, collections::HashMap, fmt, ops};

/// Nolang primitive types
#[derive(Debug, Clone)]
pub enum Primitive {
    Number(f64),
    Str(String),
    Bool(bool),
    Function(Statement, Vec<String>, HashMap<String, Primitive>),
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
            Self::Number(ref n) => n.to_string(),
            Self::Function(..) => "<function>".to_string(),
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
    type Output = f64;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Self::Number(n) => -n,
            Self::Bool(true) => -1f64,
            Self::Bool(false) => 0f64,
            _ => error!("TypeError"; "can't use `-` operator with {}", self => 1),
        }
    }
}

impl ops::Add for Primitive {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        match (&rhs, &self) {
            (Self::Number(o_num), Self::Number(s_num)) => Self::Number(o_num + s_num),
            (Self::Str(o_str), Self::Str(s_str)) => Self::Str(o_str.to_string() + s_str),
            _ => error!("TypeError"; "tried to use `+` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Sub for Primitive {
    type Output = f64;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        match (rhs.to_number(), self.to_number()) {
            (Some(o_num), Some(s_num)) => o_num - s_num,
            _ => error!("TypeError"; "tried to use `-` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Mul for Primitive {
    type Output = f64;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        match (rhs.to_number(), self.to_number()) {
            (Some(o_num), Some(s_num)) => o_num * s_num,
            _ => error!("TypeError"; "tried to use `*` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl ops::Div for Primitive {
    type Output = f64;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        match (rhs.to_number(), self.to_number()) {
            (Some(o_num), Some(s_num)) => o_num / s_num,
            _ => error!("TypeError"; "tried to use `/` operator between {} and {}", rhs, self => 1),
        }
    }
}

impl cmp::PartialEq for Primitive {
    #[inline]
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

impl cmp::PartialOrd for Primitive {
    #[inline]
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
        Primitive::Number(self)
    }
}

impl IntoPrimitive for Primitive {
    #[inline]
    fn into_pri(self) -> Primitive {
        self
    }
}
