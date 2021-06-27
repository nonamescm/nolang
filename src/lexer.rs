mod tokens;
#[macro_use]
mod util;

use tokens::{keyword_get_tok, Tokens as Tok};
use util::*;

pub struct Lexer {
    line: usize,
    pos: usize,
    raw: Vec<char>,
    ch: char,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            ch: '|',
            raw: input,
            pos: 0,
            line: 1,
        }
    }

    fn next(&mut self) {
        if self.pos < self.raw.len() {
            self.ch = self.raw[self.pos];
            self.pos += 1;
        }
    }

    fn back(&mut self) {
        self.pos -= 1;
        self.ch = self.raw[self.pos - 1];
    }

    fn ignore_comment(&mut self) -> Tok {
        while self.ch != '\n' {
            self.next();
        }
        self.line += 1;
        self.next();
        self.get_tok()
    }

    fn get_tok(&mut self) -> Tok {
        match self.ch {
            ' ' | '\r' | '\t' => Tok::Space,
            '\n' => {
                self.line += 1;
                Tok::Newline
            }

            '+' => Tok::Plus,
            '-' => Tok::Minus,
            '*' => Tok::Asterisk,
            '/' => Tok::Slash,
            '%' => Tok::Percent,
            '>' => Tok::Gt,
            '<' => Tok::Lt,
            '=' => match self.raw[self.pos] {
                '=' => {
                    self.next();
                    Tok::Comp
                }
                _ => Tok::Assign,
            },

            ',' => Tok::Comma,
            ':' => Tok::Colon,
            ';' => Tok::Semicolon,
            '.' => Tok::Point,

            '(' => Tok::Lparen,
            ')' => Tok::Rparen,
            '{' => Tok::Lbrace,
            '}' => Tok::Rbrace,

            '@' => self.ignore_comment(),
            '|' => Tok::Pipe,
            '$' => {
                self.next();
                get_val!(self; !is_ch_valid(&self.ch) => ident);

                match keyword_get_tok(&ident) {
                    Some(ident) => {
                        no_lang::err!(custom format!("keyword `{:?}` used as name on line {:?}", ident, self.line) => 1)
                    }
                    None => Tok::LocalIdent(ident),
                }
            }
            '\'' | '"' => {
                let ch = self.ch;
                self.next();
                get_val!(self; ch == self.ch => str_vec);
                self.next();
                Tok::String(str_vec)
            }
            c if is_ch_valid(&c) => {
                get_val!(self; !is_ch_valid(&self.ch) => ident);

                match keyword_get_tok(&ident) {
                    Some(v) => v,
                    None => Tok::Ident(ident),
                }
            }
            c if is_valid_math_symbol(&c) => {
                get_val!(self; !is_valid_math_symbol(&self.ch) => num);
                let val = num
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("error parsing number at line {}", self.line));
                Tok::Number(val)
            }
            _ => {
                no_lang::err!(unexpected self.ch, self.line => 1)
            }
        }
    }

    pub fn start(&mut self) -> Vec<Tok> {
        self.next();
        let mut vec_tok = vec![];
        while self.pos < self.raw.len() {
            match self.get_tok() {
                Tok::Space => (),
                t => vec_tok.push(t),
            }
            self.next();
        }
        vec_tok
    }
}
