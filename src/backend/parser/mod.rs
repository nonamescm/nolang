mod op;
mod literal;

pub use literal::Literal;
pub use op::Op;
use super::tokens::Tokens as Tok;

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
                    "ParseError: expected one of: {:?} found {:?} on line {}",
                    [ $(stringify!($tokens)),+ ],
                    $current,
                    $self.line
                ).replace("\"", "").replace("[", "").replace("]", "") => 1
            )
        }
        $self.next();
    }}
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
            consume!(eself, eself.current, Tok::Semicolon);
        }

        op_vec.into_iter()
    }

    /// Consume one token, advancing the self.current by one position
    fn next(&mut self) {
        self.index += 1;
        if (self.index as usize) < self.tokens.len() {
            if self.tokens[self.index as usize] == Tok::Newline {
                self.line += 1;
                self.next()
            } else {
                self.current = self.tokens[self.index as usize].clone()
            }
        } else {
            self.current = Tok::Eof
        }
    }

    /// Check what's the current Op
    fn check_pattern(&mut self) -> Op {
        self.equality()
    }

    /// Get raw operations
    fn primary(&mut self) -> Op {
        let literal: Literal = match &self.current {
            Tok::True => Literal::Bool(true),
            Tok::False => Literal::Bool(false),
            Tok::None => Literal::None,
            Tok::Number(n) => Literal::Number(*n),
            Tok::Str(s) => Literal::String(s.to_owned()),
            Tok::Ident(id) => Literal::VarNormal(id.to_owned()),
            Tok::LocalIdent(id) => Literal::VarLocal(id.to_owned()),
            Tok::Lparen => {
                self.next();
                let operation = self.check_pattern();
                consume!(self, self.current, Tok::Rparen);
                return Op::Grouping(Box::new(operation))
            }

            e => crate::err!(unexpected e, self.line => 1),
        };
        self.next();
        Op::Primary(Box::new(literal))
    }

    /// Get unary operations, such as `not<OP>` and `-<OP>`
    fn unary(&mut self) -> Op {
        if matches!(self.current, Tok::Minus | Tok::Not) {
            let operator = self.current.clone();
            self.next();
            let right = self.unary();
            Op::Unary(operator, Box::new(Literal::Operation(right)))
        } else {
            self.primary()
        }
    }

    /// Get multiply and division operations
    fn factor(&mut self) -> Op {
        let mut left = self.unary();

        while matches!(self.current, Tok::Asterisk | Tok::Slash) {
            let operator = self.current.clone();
            self.next();
            let right = self.unary();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }
        left
    }

    /// Get add and sub operations
    fn term(&mut self) -> Op {
        let mut left = self.factor();

        while matches!(self.current, Tok::Plus | Tok::Minus) {
            let operator = self.current.clone();
            self.next();
            let right = self.factor();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }

        left
    }

    fn comparison(&mut self) -> Op {
        let mut left = self.term();

        while matches!(self.current, Tok::Gt | Tok::GtOrEq | Tok::Lt | Tok::LtOrEq) {
            let operator = self.current.clone();
            self.next();
            let right = self.term();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }
        left
    }

    fn equality(&mut self) -> Op {
        let mut left = self.comparison();

        while matches!(self.current, Tok::Comp | Tok::Different) {
            let operator = self.current.clone();
            self.next();
            let right = self.term();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }
        left
    }
}
