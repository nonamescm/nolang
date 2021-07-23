mod lexer;
mod parser;
mod tokens;

use lexer::Lexer;
use parser::Parser;

pub fn parse(input: String) -> impl Iterator<Item = parser::Op> {
    Parser::parse(Lexer::lex(input))
}
