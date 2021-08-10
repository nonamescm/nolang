use super::super::tokens::Tokens as Tok;
use super::literal::Literal;
use super::statement::Statement;

/// Operations Enum, you can think of it as `Expr` in most parsers
#[derive(Debug, Clone)]
pub enum Op {
    Primary(Box<Literal>),
    Call(Box<Op>, Vec<Op>),
    Unary(Tok, Box<Literal>),
    Binary(Box<Op>, Tok, Box<Op>),
    Grouping(Box<Op>),
    Block(Vec<Statement>),
    If(Box<Op>, Box<Op>, Box<Op>),
}
