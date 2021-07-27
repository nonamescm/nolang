use super::Op;
#[derive(Debug, Clone)]
pub enum Statement {
    Assign(String, Box<Statement>),
    Op(Op),
    Write(Op),
    Writeln(Op),
    Block(Vec<Statement>),
    If(Op, Box<Statement>, Option<Box<Statement>>),
}
