use super::Op;

/// Var struct
#[derive(Debug, Clone)]
pub enum Var {
    VarNormal(String),
    VarLocal(String),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Var(Var),
    String(String),
    Bool(bool),
    Number(f64),
    Op(Op),
    None
}

impl Literal {
    pub fn boolean(&self) -> bool {
        match *self {
            Self::Bool(false) => false,
            Self::None => false,
            Self::Number(x) if x == 0.0 => false,
            _ => true
        }
    }
}
