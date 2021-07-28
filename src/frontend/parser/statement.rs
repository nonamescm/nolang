use super::Op;
#[derive(Debug, Clone)]
pub enum Statement {
    Assign(String, Box<Statement>),
    FuncAssign(String, Vec<String>, Box<Statement>),
    Op(Op),
    Block(Vec<Statement>),
    If(Op, Box<Statement>, Box<Statement>),
}
