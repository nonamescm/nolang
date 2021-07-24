#[derive(Debug, Clone)]
pub enum Statement {
    Assign(String, Box<Statement>),
    Op(super::Op),
    Write(Box<Statement>),
    Writeln(Box<Statement>),
}
