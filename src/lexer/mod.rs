pub mod tokens;
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
    pub fn lex(input: String) -> impl Iterator<Item = Tok> {
        let mut eself = Self {
            ch: '|',
            raw: input.chars().collect(),
            pos: 0,
            line: 1,
        };

        eself.next();
        let mut vec_tok = vec![];

        while eself.pos < eself.raw.len() {
            match eself.get_tok() {
                Tok::Space => (),
                t => vec_tok.push(t),
            }
            eself.next();
        }

        vec_tok.into_iter()
    }

    fn next(&mut self) {
        if self.pos < self.raw.len() {
            self.ch = self.current();
            self.pos += 1;
        }
    }

    fn peek(&self, ch: char) -> bool {
        self.current() == ch
    }

    fn current(&self) -> char {
        self.raw[self.pos]
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
            // Whitespaces
            ' ' | '\r' | '\t' => {
                self.next();
                self.get_tok()
            }
            '\n' => {
                self.line += 1;
                Tok::Newline
            }

            // Mathematical operators
            '+' => Tok::Plus,
            '-' => Tok::Minus,
            '*' => Tok::Asterisk,
            '/' => Tok::Slash,
            '%' => Tok::Percent,

            // Comparators
            '>' if self.peek('=') => {
                self.next();
                Tok::GtOrEq
            }
            '>' => Tok::Gt,
            '<' if self.peek('=') => {
                self.next();
                Tok::LtOrEq
            }
            '<' => Tok::Lt,
            '=' if self.peek('=') => {
                self.next();
                Tok::Comp
            }
            '=' if self.peek('>') => {
                self.next();
                Tok::ArrowAssign
            }
            '=' => Tok::Assign,

            '~' if self.peek('=') => {
                self.next();
                Tok::Different
            }
            '~' => Tok::Not,

            // Separators
            ',' => Tok::Comma,
            ';' if self.peek(';') => {
                self.next();
                Tok::End
            }
            ';' => Tok::Semicolon,
            '.' if self.peek('.') => {
                self.next();
                Tok::Concat
            }
            '.' => Tok::Point,

            // Grouping
            '(' => Tok::Lparen,
            ')' => Tok::Rparen,
            '{' => Tok::Lbrace,
            '}' => Tok::Rbrace,

            // Comment
            '@' => self.ignore_comment(),

            // Special symbols
            '|' => Tok::Pipe,

            // Identifiers and constants
            ':' => {
                self.next();
                get_val!(self; is_ch_valid(&self.ch) => ident);

                match keyword_get_tok(&ident) {
                    Some(ident) => {
                        crate::err!(custom format!("keyword `{:?}` used as name on line {:?}", ident, self.line) => 1)
                    }
                    None if !ident.is_empty() => Tok::LocalIdent(ident),
                    None => crate::err!(unexpected self.ch, self.line => 1),
                }
            }
            '\'' | '"' => {
                let ch = self.ch;
                self.next();
                get_val!(self; ch != self.ch => str_vec);
                self.next();
                Tok::String(str_vec)
            }
            c if is_valid_math_symbol(&c) => {
                get_val!(self; is_valid_math_symbol(&self.ch) => num);

                if is_ch_valid(&self.current()) {
                    crate::err!(unexpected self.current(), self.line => 1)
                }

                let val = num
                    .parse::<f64>()
                    .unwrap_or_else(|_| crate::err!(custom format!("error parsing number at line {}", self.line) => 1));
                Tok::Number(val)
            }
            c if is_ch_valid(&c) => {
                get_val!(self; is_ch_valid(&self.ch) => ident);

                match keyword_get_tok(&ident) {
                    Some(v) => v,
                    None => Tok::Ident(ident),
                }
            }

            // Nothing matches
            c => {
                crate::err!(unexpected c, self.line => 1)
            }
        }
    }
}
