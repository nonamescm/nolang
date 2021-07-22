use super::tokens::Tokens as Tok;

/// Var struct
#[derive(Debug)]
pub enum Var {
    VarNormal(String),
    VarLocal(String),
}

/// Check if a token matches and panic if it doesn't, returns ()
macro_rules! consume {
    ($current: expr, $( $tokens:pat )|+) => {
        if !matches!($current, $($tokens)|+) {
            crate::err!(
                custom
                format!(
                    "expected one of {:?} found {:?}",
                    [ $(stringify!($tokens)),+ ],
                    $current
                ) => 1
            )
        }
    };

    ($self: ident, $current: expr, $($tokens:pat)|+) => {{
        if !matches!($current, $($tokens)|+) {
            crate::err!(
                custom
                format!(
                    "expected one of {:?} found {:?}",
                    [ $(stringify!($tokens)),+ ],
                    $current
                ) => 1
            )
        }
        $self.next();
    }}
}

/// Literal values on nolang, like strings, numbers, booleans and null implement it
pub trait Literal: std::fmt::Debug {}

/// Macro to help implementing Literal trait, so you don't need to type
/// `impl Literal for ... {}` for all types
macro_rules! register_literals {
    ( $($type: ty);+ ;) => {
        $(impl Literal for $type {})+
    }
}

register_literals! {
    Var;
    String;
    bool;
    f64;
    ();
    Op;
}

/// Operations Enum, you can think of it as `Expr` in most parsers
#[derive(Debug)]
pub enum Op {
    Literal(Box<dyn Literal>),
    Assign(Var, Box<dyn Literal>),
    Unary(Tok, Box<dyn Literal>),
    Binary(Box<Op>, Tok, Box<Op>),
    Grouping(Box<Op>),
}

/// Parser struct, has the fields that are used in Parser::parse
pub struct Parser {
    index: isize,
    current: Tok,
    tokens: Vec<Tok>,
    line: usize,
}

impl Parser {
    /// Simple implementation of a parser
    pub fn parse(tokens: impl Iterator<Item = Tok>) -> impl Iterator<Item = Op> {
        let mut eself = Self {
            index: -1,
            current: Tok::None,
            tokens: tokens.collect(),
            line: 1,
        };
        eself.next();

        let mut op_vec: Vec<Op> = vec![];

        while (eself.index as usize) < eself.tokens.len() {
            op_vec.push(eself.check_pattern());
        }

        op_vec.into_iter()
    }

    /// Look at current tokens advancing by the given number
    fn look_at_by(&self, n: isize) -> &Tok {
        let n = (self.index + n) as usize;
        self.tokens.get(n).unwrap_or(&Tok::Eof)
    }

    /// Consume one token, advancing the self.current by one position
    fn next(&mut self) {
        self.index += 1;
        if (self.index as usize) < self.tokens.len() {
            self.current = self.tokens[self.index as usize].clone();
        } else {
            self.current = Tok::Eof
        }
    }

    /// Check what's the current Op
    fn check_pattern(&mut self) -> Op {
        match self.current {
            Tok::Newline => {
                self.next();
                self.check_pattern()
            }
            Tok::Lparen => self.grouping(),
            Tok::Let => self.assign(),
            Tok::LocalIdent(..) if self.look_at_by(1) == &Tok::Assign => self.assign(),

            _ if self.look_at_by(1).is_operator() => self.binary(),

            ref t if t.is_unary() | t.is_literal() => self.binary(),
            ref e => crate::err!(custom format!("{:?}: Not yet implemented", e) => 1),
        }
    }

    /// Get grouping Op: `(Op)`
    fn grouping(&mut self) -> Op {
        self.next();
        let item = Op::Grouping(Box::new(self.check_pattern()));

        consume!(self, self.current, Tok::Rparen);

        if self.current.is_operator() {
            let operator = self.current.clone();
            self.next();
            Op::Binary(Box::new(item), operator, Box::new(self.check_pattern()))
        } else {
            item
        }
    }

    fn assign(&mut self) -> Op {
        if self.current == Tok::Let {
            self.next();
        }

        let ident = Op::Literal(Box::new(match &self.current {
            Tok::Ident(id) => Var::VarNormal(id.to_owned()),
            Tok::LocalIdent(id) => Var::VarLocal(id.to_owned()),
            _ => panic!(),
        }));
        self.next();

        consume!(self, self.current, Tok::Assign);

        Op::Binary(Box::new(ident), Tok::Assign, Box::new(self.check_pattern()))
    }

    fn unary(&mut self) -> Op {
        if matches!(self.current, Tok::Minus | Tok::Not) {
            let operator = self.current.clone();
            self.next();
            let right = self.primary();
            Op::Unary(operator, Box::new(right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Op {
        let literal: Box<dyn Literal> = match &self.current {
            Tok::True => Box::new(true),
            Tok::False => Box::new(false),
            Tok::None => Box::new(()),
            Tok::Number(n) => Box::new(*n),
            Tok::Str(s) => Box::new(s.to_owned()),
            Tok::Ident(id) => Box::new(Var::VarNormal(id.to_owned())),
            Tok::LocalIdent(id) => Box::new(Var::VarLocal(id.to_owned())),

            Tok::Lparen => Box::new(self.grouping()),

            e => crate::err!(unexpected e, self.line => 1),
        };
        self.next();
        Op::Literal(literal)
    }

    fn binary(&mut self) -> Op {
        let right = self.unary();

        if self.current.is_operator() {
            let operator = self.current.clone();
            self.next();
            let left = self.check_pattern();

            Op::Binary(Box::new(right), operator, Box::new(left))
        } else {
            right
        }
    }
}
