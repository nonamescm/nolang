mod lexer;
mod parser;
mod tokens;

use lexer::Lexer;
use parser::Parser;
pub use {
    parser::{Literal, Op, Statement},
    tokens::Tokens,
};

pub fn parse(input: String) -> impl Iterator<Item = Statement> {
    Parser::parse(Lexer::lex(input))
}

pub fn lex(input: String) -> impl Iterator<Item = Tokens> {
    Lexer::lex(input)
}
