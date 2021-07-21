mod parser;
mod lexer;
mod tokens;

use parser::Parser;
use lexer::Lexer;

pub fn parse(input: String) -> impl Iterator<Item = Box<dyn parser::Op>> {
    Parser::parse(Lexer::lex(input))
}
