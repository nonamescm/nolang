use super::super::tokens::Tokens as Tok;
use super::literal::Literal;

/// Operations Enum, you can think of it as `Expr` in most parsers
#[derive(Debug, Clone)]
pub enum Op {
    Primary(Box<Literal>),
    Unary(Tok, Box<Literal>),
    Binary(Box<Op>, Tok, Box<Op>),
    Grouping(Box<Op>),
}
