#[macro_use]
mod primitive;
use primitive::Primitive;
use std::collections::HashMap;

use crate::backend::{Literal, Statement, Op, Tokens as Tok};

pub struct Interpreter {
    statements: Vec<Statement>,
    index: usize,
    variables: HashMap<String, Primitive>
}

impl Interpreter {
    pub fn interpret(operations: impl Iterator<Item = Statement>) {
        let mut eself = Self {
            statements: operations.collect(),
            index: 0,
            variables: HashMap::new()
        };

        loop {
            eself.statement(eself.statements[eself.index].clone());
            if !eself.next(){ break }
        }
    }

    fn next(&mut self) -> bool {
        self.index += 1;

        self.index < self.statements.len()
    }

    fn statement(&mut self, statement: Statement) -> Primitive {
        match statement {
            Statement::Op(op) => self.evaluate(op),
            Statement::Write(value) => {
                print!("{}", self.statement(*value));
                Primitive::None
            }
            Statement::Writeln(value) => {
                println!("{}", self.statement(*value));
                Primitive::None
            }
            Statement::Assign(var, value) => {
                let value = self.statement(*value);
                self.variables.insert(var, value);
                Primitive::None
            }
            #[allow(unreachable_patterns)]
            _ => unimplemented!()
        }
    }

    /// Eval primary expressions, that are just the minimal possible expression
    fn eval_primary(&mut self, prim: Literal) -> Primitive {
        match prim {
            Literal::Bool(b) => Primitive::Bool(b),
            Literal::None => Primitive::None,
            Literal::String(ref s) => Primitive::Str(s.clone()),
            Literal::Operation(ref op) => self.evaluate(op.clone()),
            Literal::Number(n) => Primitive::Number(n),
            Literal::VarNormal(v) => {
                (
                    *self.variables.get(&v).unwrap_or_else(|| {
                        error!("│ RuntimeError:\n│ acessing undefined variable {}", v => 1)
                    })
                ).clone()
            }
            _ => todo!() // I still not implemented variables
        }
    }

    /// Unary expression evaluator
    fn eval_unary(&mut self, op: &Tok, right: Literal) -> Primitive {
        match op {
            Tok::Minus => -self.evaluate(Op::Primary(Box::new(right))),
            Tok::Not => Primitive::Bool(!self.evaluate(Op::Primary(Box::new(right)))),
            _ => unreachable!()
        }
    }

    /// binary expression evaluator, like `1+1` or `1*1`
    fn eval_binary(&mut self, left: Op, op: &Tok, right: Op) -> Primitive {
        match op {
            // operations
            Tok::Plus => self.evaluate(right) + self.evaluate(left),
            Tok::Minus => self.evaluate(right) - self.evaluate(left),
            Tok::Asterisk => self.evaluate(right) * self.evaluate(left),
            Tok::Slash => self.evaluate(right) / self.evaluate(left),

            // Comparisons
            Tok::Comp => Primitive::Bool(self.evaluate(right) == self.evaluate(left)),
            Tok::Different => Primitive::Bool(self.evaluate(right) != self.evaluate(left)),
            Tok::Gt => Primitive::Bool(self.evaluate(right) > self.evaluate(left)),
            Tok::GtOrEq => Primitive::Bool(self.evaluate(right) >= self.evaluate(left)),
            Tok::Lt => Primitive::Bool(self.evaluate(right) < self.evaluate(left)),
            Tok::LtOrEq => Primitive::Bool(self.evaluate(right) <= self.evaluate(left)),
            _ => unreachable!()
        }
    }

    /// Minimal wrapper that sends the Op to the correct evaluator
    fn evaluate(&mut self, operation: Op) -> Primitive {
        match operation {
            Op::Primary(ref value) => self.eval_primary((**value).clone()),
            Op::Unary(ref op, ref right) => self.eval_unary(op, (**right).clone()),
            Op::Binary(ref left, ref op, ref right) => self.eval_binary((**left).clone(), op, (**right).clone()),
            Op::Grouping(ref op) => self.evaluate(*op.clone())
        }
    }
}
