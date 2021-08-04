use super::Op;

#[derive(Debug, Clone)]
pub enum Literal {
    VarNormal(String),
    String(String),
    Bool(bool),
    Int(i32),
    Float(f64),
    Operation(Op),
    None,
}

impl Literal {
    pub fn boolean(&self) -> bool {
        match *self {
            Self::Bool(false) => false,
            Self::None => false,
            Self::Float(x) if x == 0.0 => false,
            _ => true,
        }
    }
}
