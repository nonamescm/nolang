mod op;
mod literal;
mod statement;

pub use literal::Literal;
pub use statement::Statement;
pub use op::Op;
use super::tokens::Tokens as Tok;

/// Check if a token matches and panic if it doesn't, returns ()
macro_rules! consume {
    ($current: expr, $( $tokens:pat )|+) => {{
        let printable = format!("{:?}", [ $(stringify!($tokens)),+ ]).replace("\"", "")
            .replace("[", "")
            .replace("]", "");
        if !matches!($current, $($tokens)|+) {
            crate::error!("ParseError"; "expected one of: {} found {:?}", printable, $current => 1)
        } else { true }
    }};

    ($self: ident, $current: expr, $($tokens:pat)|+) => {{
        consume!($current, $($tokens)|+);
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
    pub fn parse(tokens: impl Iterator<Item = Tok>) -> impl Iterator<Item = Statement> {
        let mut eself = Self {
            index: -1,
            current: Tok::None,
            tokens: tokens.collect(),
            line: 1,
        };
        eself.next();

        let mut staments_vec: Vec<Statement> = vec![];

        while (eself.index as usize) < eself.tokens.len() {
            staments_vec.push(eself.check_statement());
            consume!(eself, eself.current, Tok::Semicolon);
        }

        staments_vec.into_iter()
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

    fn check_statement(&mut self) -> Statement {
        match self.current {
            Tok::Write => self.write_statement(),
            Tok::Writeln => self.writeln_statement(),
            Tok::Let => self.assign_statement(),
            _ => Statement::Op(self.check_pattern())
        }
    }

    fn write_statement(&mut self) -> Statement {
        self.next();
        let to_write = self.check_statement();
        Statement::Write(Box::new(to_write))
    }

    fn writeln_statement(&mut self) -> Statement {
        self.next();
        let to_write = self.check_statement();
        Statement::Writeln(Box::new(to_write))
    }

    fn assign_statement(&mut self) -> Statement {
        self.next();
        if consume!(self.current, Tok::Ident(..)) {
            let var_name = match &self.current {
                Tok::Ident(id) => id.clone(),
                _ => unreachable!()
            };
            self.next();

            consume!(self, self.current, Tok::Assign);
            let value = self.check_statement();
            Statement::Assign(var_name, Box::new(value))
        } else { unreachable!() }
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

            e => crate::error!("ParseError"; "Unexpected `{}` on line {}", e, self.line => 1),
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
