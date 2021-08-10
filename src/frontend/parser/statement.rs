use super::Op;
#[derive(Debug, Clone)]
pub enum Statement {
    Assign(String, Box<Op>),
    FuncAssign(String, Vec<String>, Box<Statement>),
    Op(Op),
}
