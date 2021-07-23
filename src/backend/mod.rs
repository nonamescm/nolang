mod lexer;
mod parser;
mod tokens;

pub use {
    parser::{Var, Op, Literal},
    tokens::Tokens
};
use lexer::Lexer;
use parser::Parser;

pub fn parse(input: String) -> impl Iterator<Item = Op> {
    Parser::parse(Lexer::lex(input))
}
