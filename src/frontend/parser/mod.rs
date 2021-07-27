mod literal;
mod op;
mod statement;

use super::tokens::Tokens as Tok;
pub use literal::Literal;
pub use op::Op;
pub use statement::Statement;

/// Check if a token matches and panic if it doesn't, returns ()
macro_rules! consume {
    ($current: expr, $( $tokens:pat )|+) => {{
        let printable = format!("{:?}", [ $(stringify!($tokens)),+ ]).replace("\"", "")
            .replace("[", "")
            .replace("]", "");
        if !matches!($current, $($tokens)|+) {
            crate::error!("ParseError"; "expected one of: {} found {:?}", printable, $current => 1)
        }
        true
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
            staments_vec.push(eself.statement());
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

    fn statement(&mut self) -> Statement {
        let operation = match self.current {
            Tok::Write => self.write_stat(),
            Tok::Writeln => self.writeln_stat(),
            Tok::Let => self.assign_stat(),
            Tok::Do => self.block_stat(),
            Tok::If => self.if_stat(),
            _ => {
                let x = Statement::Op(self.operation());
                consume!(self, self.current, Tok::Semicolon);
                x
            }
        };
        operation
    }

    fn if_stat(&mut self) -> Statement {
        let line = self.line;
        self.next(); // skips the current `if` toke
        let condition = self.operation();
        consume!(self, self.current, Tok::Semicolon);
        let body = self.statement();
        
        if matches!(body, Statement::Assign(..)) {
            crate::error!("SyntaxError"; "tried to declare a variable from a if on line {}", line => 1)
        }

        match self.current {
            Tok::Else => {
                self.next();
                let else_body = self.statement();
                Statement::If(condition, Box::new(body), Some(Box::new(else_body)))
            }
            Tok::Elif => {
                Statement::If(condition, Box::new(body), Some(Box::new(self.if_stat())))
            }
            _ => Statement::If(condition, Box::new(body), None)
        }

    }

    fn block_stat(&mut self) -> Statement {
        let line = self.line;
        let mut vec_stat = vec![];
        self.next();

        while !matches!(self.current, Tok::Done) {
            vec_stat.push(self.statement());

            if matches!(self.current, Tok::Eof) {
                crate::error!("ParseError"; "unclosed do block opened on line {}", line => 1)
            }
        }
        consume!(self.current, Tok::Done);
        self.next();

        consume!(self, self.current, Tok::Semicolon);
        self.next();

        Statement::Block(vec_stat)
    }

    fn write_stat(&mut self) -> Statement {
        self.next();
        let to_write = self.operation();
        consume!(self, self.current, Tok::Semicolon);

        Statement::Write(to_write)
    }

    fn writeln_stat(&mut self) -> Statement {
        self.next();
        let to_write = self.operation();
        consume!(self, self.current, Tok::Semicolon);

        Statement::Writeln(to_write)
    }

    fn assign_stat(&mut self) -> Statement {
        self.next();
        if consume!(self.current, Tok::Ident(..)) {
            let var_name = match &self.current {
                Tok::Ident(id) => id.to_string(),
                _ => unreachable!(),
            };
            self.next();
            consume!(self, self.current, Tok::Assign);

            let value = self.statement();

            Statement::Assign(var_name, Box::new(value))
        } else {
            unreachable!()
        }
    }

    /// Check what's the current Op
    fn operation(&mut self) -> Op {
        self.equality_op()
    }

    /// Get raw operations
    fn primary_op(&mut self) -> Op {
        let literal: Literal = match &self.current {
            Tok::True => Literal::Bool(true),
            Tok::False => Literal::Bool(false),
            Tok::None => Literal::None,
            Tok::Number(n) => Literal::Number(*n),
            Tok::Str(s) => Literal::String(s.to_string()),
            Tok::Ident(id) => Literal::VarNormal(id.to_string()),
            Tok::Lparen => {
                self.next();
                let operation = self.operation();
                consume!(self, self.current, Tok::Rparen);
                return Op::Grouping(Box::new(operation));
            }

            e => crate::error!("ParseError"; "Unexpected `{}` on line {}", e, self.line => 1),
        };
        self.next();
        Op::Primary(Box::new(literal))
    }

    fn call_op(&mut self) -> Op {
        let mut called = self.primary_op();

        while matches!(self.current, Tok::Lparen) {
            self.next();

            let mut arguments = Vec::new();
            while !matches!(self.current, Tok::Rparen) {
                arguments.push(self.equality_op());

                if matches!(self.current, Tok::Rparen) {
                    break;
                }

                consume!(self, self.current, Tok::Comma);
            }
            self.next();
            match called {
                Op::Primary(ref p) => match **p {
                    Literal::VarNormal(..) => (),
                    _ => crate::error!("TypeError"; "Can't call `{:?}`", p => 1)
                }
                Op::Call(..) => (),
                _ => crate::error!("TypeError"; "Can't call `{:?}`", called => 1)
            }
            called = Op::Call(Box::new(called), arguments)
        }
        called
    }

    /// Get unary operations, such as `not(<OP>)` and `-<OP>`
    fn unary_op(&mut self) -> Op {
        if matches!(self.current, Tok::Minus | Tok::Not) {
            let operator = self.current.clone();
            self.next();
            let right = self.call_op();
            Op::Unary(operator, Box::new(Literal::Operation(right)))
        } else {
            self.call_op()
        }
    }

    /// Get multiply and division operations
    fn factor_op(&mut self) -> Op {
        let mut left = self.unary_op();

        while matches!(self.current, Tok::Asterisk | Tok::Slash) {
            let operator = self.current.clone();
            self.next();
            let right = self.unary_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }
        left
    }

    /// Get add and sub operations
    fn term_op(&mut self) -> Op {
        let mut left = self.factor_op();

        while matches!(self.current, Tok::Plus | Tok::Minus) {
            let operator = self.current.clone();
            self.next();
            let right = self.factor_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }

        left
    }

    fn and_op(&mut self) -> Op {
        let mut left = self.term_op();

        while matches!(self.current, Tok::And) {
            let operator = self.current.clone();
            self.next();
            let right = self.factor_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }

        left
    }

    fn or_op(&mut self) -> Op {
        let mut left = self.and_op();

        while matches!(self.current, Tok::Or) {
            let operator = self.current.clone();
            self.next();
            let right = self.and_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }

        left
    }

    fn comparison_op(&mut self) -> Op {
        let mut left = self.or_op();

        while matches!(self.current, Tok::Gt | Tok::GtOrEq | Tok::Lt | Tok::LtOrEq) {
            let operator = self.current.clone();
            self.next();
            let right = self.or_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }
        left
    }

    fn equality_op(&mut self) -> Op {
        let mut left = self.comparison_op();

        while matches!(self.current, Tok::Comp | Tok::Different) {
            let operator = self.current.clone();
            self.next();
            let right = self.term_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }
        left
    }
}
