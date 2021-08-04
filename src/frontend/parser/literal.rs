use super::Op;

#[derive(Debug, Clone)]
pub enum Literal {
    VarNormal(String),
    String(String),
    Bool(bool),
    Num(f64),
    Operation(Op),
    None,
}

impl Literal {
    pub fn boolean(&self) -> bool {
        match *self {
            Self::Bool(false) => false,
            Self::None => false,
            Self::Num(x) if x == 0.0 => false,
            _ => true,
        }
    }
}
