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
        let printable = format!("{:?}", [ $(stringify!($tokens).split("::").last().unwrap()),+ ]).replace("\"", "")
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
        true
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
            consume!(eself, eself.current, Tok::Eof | Tok::Newline | Tok::Semicolon);
        }

        staments_vec.into_iter()
    }

    /// Consume one token, advancing the self.current by one position
    fn next(&mut self) {
        self.index += 1;
        if (self.index as usize) < self.tokens.len() {
            if self.tokens[self.index as usize] == Tok::Newline {
                self.line += 1;
            }
            self.current = self.tokens[self.index as usize].clone()
        } else {
            self.current = Tok::Eof
        }
    }

    fn next_skip(&mut self) {
        loop {
            self.next();
            if !matches!(self.current, Tok::Newline) {
                break;
            }
        }
    }


    // Statements region
    fn statement(&mut self) -> Statement {
        while matches!(self.current, Tok::Newline | Tok::Semicolon) {
            self.next()
        }
        match self.current {
            Tok::Let => self.assign_stat(),
            Tok::Defn => self.defn_stat(),
            _ => Statement::Op(self.operation())
        }
    }

    fn defn_stat(&mut self) -> Statement {
        consume!(self, self.current, Tok::Defn);
        consume!(self, self.current, Tok::Lparen);

        let mut arguments = Vec::new();

        while !matches!(self.current, Tok::Rparen) {
            arguments.push(match &self.current {
                Tok::Ident(id) => id.to_string(),
                _ => crate::error!("ParseError"; "Expected variable name after function on line {}", self.line => 1),
            });
            self.next_skip();

            if !matches!(self.current, Tok::Rparen) {
                consume!(self, self.current, Tok::Comma);
            }
        }
        consume!(self, self.current, Tok::Rparen);

        let name = match &self.current {
            Tok::Ident(id) => id.to_string(),
            e => crate::error!("ParseError"; "expected ident after function declaration, found {}, on line {}", e, self.line => 1)
        };
        self.next_skip();
        consume!(self, self.current, Tok::Assign);
        let block = Box::new(self.statement());

        Statement::FuncAssign(name, arguments, block)
    }

    fn assign_stat(&mut self) -> Statement {
        self.next_skip();
        consume!(self.current, Tok::Ident(..));

        let var_name = match &self.current {
            Tok::Ident(id) => id.to_string(),
            _ => unreachable!(),
        };
        self.next_skip();

        if consume!(self, self.current, Tok::Assign) {
            let value = self.operation();

            Statement::Assign(var_name, Box::new(value))
        } else { unreachable!() }
    }
    // End statements region

    // Operations Region
    fn if_op(&mut self) -> Op {
        self.next_skip(); // skips the current `if` toke
        let condition = self.operation();
        consume!(self, self.current, Tok::Then);
        let body = self.operation();

        match self.current {
            Tok::Else => {
                self.next_skip();
                let else_body = self.operation();
                Op::If(Box::new(condition), Box::new(body), Box::new(else_body))
            }
            Tok::Elif => Op::If(Box::new(condition), Box::new(body), Box::new(self.if_op())),
            _ => crate::error!("ParseError"; "expected `else` after if" => 1),
        }
    }

    fn block_op(&mut self) -> Op {
        let line = self.line;
        let mut vec_stat = vec![];
        self.next_skip();

        while !matches!(self.current, Tok::End) {
            vec_stat.push(self.statement());

            if matches!(self.current, Tok::Eof) {
                crate::error!("ParseError"; "unclosed do block opened on line {}", line => 1)
            }
        }
        consume!(self, self.current, Tok::End);
        consume!(self, self.current, Tok::Semicolon | Tok::Newline | Tok::Eof);

        Op::Block(vec_stat)
    }

    /// Check what's the current Op
    fn operation(&mut self) -> Op {
        match self.current {
            Tok::If => self.if_op(),
            Tok::Do => self.block_op(),
            _ => self.equality_op()
        }
    }

    /// Get raw operations
    fn primary_op(&mut self) -> Op {
        while matches!(self.current, Tok::Newline | Tok::Semicolon) {
            self.next()
        }
        let literal: Literal = match &self.current {
            Tok::True => Literal::Bool(true),
            Tok::False => Literal::Bool(false),
            Tok::None => Literal::None,
            Tok::Num(n) => Literal::Num(*n),
            Tok::Str(s) => Literal::String(s.to_string()),
            Tok::Ident(id) => Literal::VarNormal(id.to_string()),
            Tok::Lparen => {
                self.next_skip();
                let operation = self.operation();
                consume!(self, self.current, Tok::Rparen);
                return Op::Grouping(Box::new(operation));
            }

            e => crate::error!("ParseError"; "Unexpected `{}` on line {}", e, self.line => 1),
        };
        self.next_skip();
        Op::Primary(Box::new(literal))
    }

    fn call_op(&mut self) -> Op {
        let mut called = self.primary_op();

        while matches!(self.current, Tok::Lparen) {
            self.next_skip();

            let mut arguments = Vec::new();
            while !matches!(self.current, Tok::Rparen) {
                arguments.push(self.operation());

                if matches!(self.current, Tok::Rparen) {
                    break;
                }

                consume!(self, self.current, Tok::Comma);
            }
            self.next_skip();
            match called {
                Op::Primary(ref p) => match **p {
                    Literal::VarNormal(..) => (),
                    _ => crate::error!("TypeError"; "Can't call `{:?}`", p => 1),
                },
                Op::Call(..) => (),
                _ => crate::error!("TypeError"; "Can't call `{:?}`", called => 1),
            }
            called = Op::Call(Box::new(called), arguments)
        }
        called
    }

    /// Get unary operations, such as `not(<OP>)` and `-<OP>`
    fn unary_op(&mut self) -> Op {
        if matches!(self.current, Tok::Minus | Tok::Not) {
            let operator = self.current.clone();
            self.next_skip();
            let right = self.call_op();
            Op::Unary(operator, Box::new(Literal::Operation(right)))
        } else {
            self.call_op()
        }
    }

    /// Get multiply and division operations
    fn factor_op(&mut self) -> Op {
        let mut left = self.unary_op();

        while matches!(self.current, Tok::Asterisk | Tok::Slash | Tok::Percent | Tok::Pow) {
            let operator = self.current.clone();
            self.next_skip();
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
            self.next_skip();
            let right = self.factor_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }

        left
    }

    fn and_op(&mut self) -> Op {
        let mut left = self.term_op();

        while matches!(self.current, Tok::And) {
            let operator = self.current.clone();
            self.next_skip();
            let right = self.factor_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }

        left
    }

    fn or_op(&mut self) -> Op {
        let mut left = self.and_op();

        while matches!(self.current, Tok::Or) {
            let operator = self.current.clone();
            self.next_skip();
            let right = self.and_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }

        left
    }

    fn comparison_op(&mut self) -> Op {
        let mut left = self.or_op();

        while matches!(self.current, Tok::Gt | Tok::GtOrEq | Tok::Lt | Tok::LtOrEq) {
            let operator = self.current.clone();
            self.next_skip();
            let right = self.or_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }
        left
    }

    fn equality_op(&mut self) -> Op {
        let mut left = self.comparison_op();

        while matches!(self.current, Tok::Comp | Tok::Different) {
            let operator = self.current.clone();
            self.next_skip();
            let right = self.term_op();

            left = Op::Binary(Box::new(left), operator, Box::new(right))
        }
        left
    }
}
