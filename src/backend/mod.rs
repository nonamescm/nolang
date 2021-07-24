mod lexer;
mod parser;
mod tokens;

pub use {
    parser::{Op, Literal, Statement},
    tokens::Tokens
};
use lexer::Lexer;
use parser::Parser;

pub fn parse(input: String) -> impl Iterator<Item = Statement> {
    Parser::parse(Lexer::lex(input))
}

pub fn lex(input: String) -> impl Iterator<Item = Tokens> {
    Lexer::lex(input)
}
