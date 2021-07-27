use super::super::tokens::Tokens as Tok;
use super::literal::Literal;
use super::Statement;

/// Operations Enum, you can think of it as `Expr` in most parsers
#[derive(Debug, Clone)]
pub enum Op {
    Primary(Box<Literal>),
    Call(
        Box<Op>, // called
        Vec<Op>, // arguments
    ),
    Unary(Tok, Box<Literal>),
    Binary(Box<Op>, Tok, Box<Op>),
    Grouping(Box<Op>),

    If(
        Box<Op>, // the conditional
        Box<Statement>, // the body
        Option<Box<Statement>>, // else block
    ),
}
