use super::Op;
#[derive(Debug, Clone)]
pub enum Statement {
    Assign(String, Op),
    Op(Op),
    Write(Op),
    Writeln(Op),
    Block(Vec<Statement>),
    If(
        Op, // the conditional
        Box<Statement>, // the body
        Option<Box<Statement>>, // else block
    ),
}
