mod tokens;
use tokens::{keyword_get_tok, Tokens as Tok};

macro_rules! get_val {
    ($self:expr; $cond:expr => $create:ident) => {
        let mut $create = vec![];
        loop {
            $create.push($self.ch);
            $self.next();
            if $cond {
                break;
            }
        }
    };
}

fn is_ch_valid(c: &char) -> bool {
    c.is_alphabetic() || c == &'_'
}

pub struct Scanner {
    line: usize,
    pos: usize,
    raw: Vec<char>,
    ch: char,
}

impl Scanner {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            ch: '#',
            raw: input,
            pos: 0,
            line: 1,
        }
    }

    fn next(&mut self) {
        self.ch = match self.pos >= self.raw.len() {
            true => '#',
            _ => self.raw[self.pos],
        };

        self.pos += 1;
    }

    fn back(&mut self) {
        self.pos -= 1;
        self.ch = self.raw[self.pos - 1];
    }

    fn ignore_comment(&mut self) -> Tok {
        while self.ch != '\n' && self.ch != '#' {
            self.next();
        }
        self.next();
        self.get_tok()
    }

    fn get_tok(&mut self) -> Tok {
        match self.ch {
            '#' => Tok::EOF,

            ' ' | '\r' | '\t' => Tok::SPACE,
            '\n' => {
                self.line += 1;
                Tok::NEWLINE
            }

            '+' => Tok::PLUS,
            '-' => Tok::MINUS,
            '*' => Tok::ASTERISK,
            '/' => Tok::SLASH,
            '%' => Tok::PERCENT,
            '>' => Tok::GT,
            '<' => Tok::LT,
            '=' => match self.raw[self.pos + 1] {
                '=' => {
                    self.next();
                    Tok::COMP
                }
                _ => Tok::ASSIGN,
            },

            ',' => Tok::COMMA,
            ':' => Tok::COLON,
            ';' => Tok::SEMICOLON,

            '(' => Tok::LPAREN,
            ')' => Tok::RPAREN,
            '{' => Tok::LBRACE,
            '}' => Tok::RBRACE,

            '@' => self.ignore_comment(),
            '|' => Tok::PIPE,
            '$' => Tok::DOLLAR,
            '\'' | '"' => {
                let ch = self.ch;
                self.next();
                get_val!(self; ch == self.ch||self.ch == '#' => str_vec);
                Tok::STRING(str_vec)
            }
            _ => {
                if is_ch_valid(&self.ch) {
                    get_val!(self; !is_ch_valid(&self.ch) => ident);
                    self.back();
                    match keyword_get_tok(&ident) {
                        Some(v) => v,
                        None => Tok::IDENT(ident),
                    }
                } else if self.ch.is_numeric() {
                    get_val!(self; !self.ch.is_numeric() => num);
                    self.back();
                    Tok::NUM(num)
                } else {
                    no_lang::err!(self.ch, self.line => 0)
                }
            }
        }
    }

    pub fn start(&mut self) -> Vec<Tok> {
        self.next();
        let mut vec_tok = vec![];
        loop {
            let tok = match self.get_tok() {
                Tok::EOF => break,
                e => e,
            };
            match tok {
                Tok::SPACE => (),
                _ => vec_tok.push(tok),
            }
            self.next();
        }
        vec_tok
    }
}
