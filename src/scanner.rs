mod tokens;
use tokens::{
    keyword_get_tok,
    Tokens as Tok
};

fn is_valid_letter(c: &char) -> bool {
    c.is_alphabetic() || c == &'_'
}

pub struct Scanner {
    // line: usize,
    pos: usize,
    raw: Vec<char>,
    ch: char
}

impl Scanner {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            ch: 'ł',
            raw: input,
            pos: 0,
            // line: 0
        }
    }

    fn next(&mut self) {
        self.ch = match self.pos >= self.raw.len() {
            true => 'ł',
            _ => self.raw[self.pos]
        };

        self.pos += 1;
    }

    fn back(&mut self) {
        self.pos -= 1;
        self.ch = self.raw[self.pos - 1];
    }

    fn get_val(&mut self) -> Tok {
        let val = if is_valid_letter(&self.ch) {
            let mut ident = vec![];
            loop {
                ident.push(self.ch);
                self.next();
                if !is_valid_letter(&self.ch){
                    break
                }
            }
            match keyword_get_tok(&ident) {
                Some(v) => v,
                None => Tok::IDENT(ident)
            }
        } else if self.ch.is_numeric() {
            let mut num = vec![];
            loop {
                num.push(self.ch);
                self.next();
                if self.ch.is_numeric(){
                    break
                }
            }
            Tok::NUM(num)
        } else {
            Tok::UNKNOWN
        };
        self.back();
        val
    }

    fn ignore_comment(&mut self) -> Tok {
        let mut last_tok = self.get_tok();
        loop {
            match last_tok {
                Tok::NEWLINE|Tok::EOF => break,
                _ => last_tok = self.get_tok()
            }
        }
        last_tok
    }

    fn get_tok(&mut self) -> Tok {
        match self.ch {
            'ł' => Tok::EOF,
            ' '|'\r'|'\t' => Tok::SPACE,
            '\n' => Tok::NEWLINE,
            '#' => self.ignore_comment(),

            '+' => Tok::PLUS,
            '-' => Tok::MINUS,
            '*' => Tok::ASTERISK,
            '/' => Tok::SLASH,
            '%' => Tok::PERCENT,
            '>' => Tok::GT,
            '<' => Tok::LT,
            '=' => match self.raw[self.pos + 1] {
                '=' => { self.next(); Tok::COMP },
                _ => Tok::ASSIGN
            },

            ',' => Tok::COMMA,
            ':' => Tok::COLON,
            ';' => Tok::SEMICOLON,

            '(' => Tok::LPAREN,
            ')' => Tok::RPAREN,
            '{' => Tok::LBRACE,
            '}' => Tok::RBRACE,

            '@' => Tok::COMMENT,
            '|' => Tok::PIPE,
            '$' => Tok::DOLLAR,
            '\''|'"' => {
                let ch = self.ch;
                let mut str_vec = vec![];
                self.next();
                while self.ch != ch {
                    str_vec.push(self.ch);
                    self.next()
                }
                Tok::STRING(str_vec)
            }
            _ => self.get_val()
        }
    }

    pub fn start(&mut self) -> Vec<Tok> {
        self.next();
        let mut vec_tok = vec![];
        loop {
            let tok = match self.get_tok() {
                Tok::EOF => break,
                Tok::UNKNOWN => panic!("unrecognized token"),
                e => e
            };
            match tok {
                Tok::SPACE => (),
                _ => vec_tok.push(tok)
            }
            self.next();
        }
        vec_tok
    }
}
