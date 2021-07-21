use super::tokens::Tokens as Tok;
use Tok::{
    Let,
    Ident,
    Assign,
    Str,
    Number
};

pub struct Parser {
    index: isize,
    current: Tok,
    tokens: Vec<Tok>,
    line: usize,
}

macro_rules! consume {
    ($current: expr, $( $tokens:pat )|+) => {
        assert!( matches!($current, $($tokens)|+ ) )
    }
}

pub trait Op : std::fmt::Debug {}

#[derive(Debug)]
struct AssignOp(String, Box<dyn Op>);
impl Op for f64 {}
impl Op for String {}
impl Op for AssignOp {}

impl Parser {
    pub fn parse(tokens: impl Iterator<Item = Tok>) -> impl Iterator<Item = Box<dyn Op>> {
        let mut eself = Self { index: -1, current: Tok::None, tokens: tokens.collect(), line: 1 };
        eself.next();

        let mut op_vec: Vec<Box<(dyn Op + 'static)>> = vec![];

        while (eself.index as usize) < eself.tokens.len() {
            op_vec.push( Box::new(eself.check_pattern()) );
        }

        op_vec.into_iter()
    }

    fn next(&mut self) {
        self.index += 1;
        if (self.index as usize) < self.tokens.len() {
            self.current = self.tokens[self.index as usize].clone();
        }
    }

    fn check_pattern(&mut self) -> impl Op {
        let x = match self.current {
            Let => self.assign(),
            _ => todo!()
        };
        self.next();
        x
    }

    fn assign(&mut self) -> AssignOp {
        self.next();

        consume!(self.current, Ident(..));
        let ident = match &self.current {
            Ident(id) => id.to_owned(),
            _ => panic!()
        };

        self.next();
        assert!(matches!(self.current, Assign));

        self.next();
        match &self.current {
            Number(x) => AssignOp( ident, Box::new(x.clone()) ),
            Str(s) => AssignOp( ident, Box::new(s.clone()) ),
            e => crate::err!(unexpected e, self.line => 1)
        }
    }
}
