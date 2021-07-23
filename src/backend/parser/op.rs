use super::literal::Literal;
use super::super::tokens::Tokens as Tok;

/// Operations Enum, you can think of it as `Expr` in most parsers
#[derive(Debug)]
pub enum Op {
    Literal(Box<Literal>),
    Unary(Tok, Box<Literal>),
    Binary(Box<Op>, Tok, Box<Op>),
    Grouping(Box<Op>),
}
