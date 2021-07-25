#[macro_use]
mod primitive;
use primitive::Primitive;
use std::collections::HashMap;

use crate::backend::{Literal, Op, Statement, Tokens as Tok};

pub fn interpret(operations: impl Iterator<Item = Statement>) {
    let mut runtime = Interpreter {
        statements: operations.collect(),
        index: 0,
        variables: HashMap::new(),
    };

    loop {
        runtime.statement(runtime.statements[runtime.index].clone());
        if !runtime.next() {
            break;
        }
    }
}

/// The Interpreter implementation
struct Interpreter {
    statements: Vec<Statement>,
    index: usize,
    variables: HashMap<String, Primitive>,
}

impl Interpreter {
    /// Advance the index by one
    fn next(&mut self) -> bool {
        self.index += 1;

        self.index < self.statements.len()
    }

    fn statement(&mut self, statement: Statement) -> Primitive {
        match statement {
            Statement::Op(op) => self.evaluate(op),
            Statement::Write(value) => self.s_eval_write(value),
            Statement::Writeln(value) => self.s_eval_writeln(value),
            Statement::Assign(var, value) => self.s_eval_assign(var, value),
            Statement::Block(statements) => self.s_eval_block(statements),
            #[allow(unreachable_patterns)]
            // for when I implement new statements and want to test them on the parser
            _ => unimplemented!(),
        }
    }

    fn s_eval_block(&mut self, statements: Vec<Statement>) -> Primitive {
        for st in statements.into_iter() {
            self.statement(st);
        }
        Primitive::None
    }

    /// `write <OP>;` statement evaluator
    fn s_eval_write(&mut self, value: Op) -> Primitive {
        print!("{}", self.evaluate(value));
        Primitive::None
    }

    /// `writeln <OP>;` statement evaluator
    fn s_eval_writeln(&mut self, value: Op) -> Primitive {
        println!("{}", self.evaluate(value));
        Primitive::None
    }

    /// Assignment `let x = <OP>;` evaluator
    fn s_eval_assign(&mut self, var: String, value: Op) -> Primitive {
        let value = self.evaluate(value);
        if self.variables.get(&var).is_some() {
            crate::error!("TypeError"; "tried to reassign global constant {}", var => 1)
        }
        self.variables.insert(var, value);
        Primitive::None
    }

    /// Eval primary expressions, that are just the minimal possible expression
    fn eval_primary(&mut self, prim: Literal) -> Primitive {
        match prim {
            Literal::Bool(b) => Primitive::Bool(b),
            Literal::None => Primitive::None,
            Literal::String(ref s) => Primitive::Str(s.to_string()),
            Literal::Operation(ref op) => self.evaluate(op.clone()),
            Literal::Number(n) => Primitive::Number(n),
            Literal::VarNormal(v) => (*self.variables.get(&v).unwrap_or_else(
                || crate::error!("RuntimeError"; "acessing undefined variable {}", v => 1),
            ))
            .to_owned(),
            _ => todo!(), // I still not implemented variables
        }
    }

    /// Unary expression evaluator
    fn eval_unary(&mut self, op: &Tok, right: Literal) -> Primitive {
        match op {
            Tok::Minus => -self.evaluate(Op::Primary(Box::new(right))),
            Tok::Not => Primitive::Bool(!self.evaluate(Op::Primary(Box::new(right)))),
            _ => unreachable!(),
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
            _ => unreachable!(),
        }
    }

    /// Minimal wrapper that sends the Op to the correct evaluator
    fn evaluate(&mut self, operation: Op) -> Primitive {
        match operation {
            Op::Primary(ref value) => self.eval_primary((**value).clone()),
            Op::Unary(ref op, ref right) => self.eval_unary(op, (**right).clone()),
            Op::Binary(ref left, ref op, ref right) => {
                self.eval_binary((**left).clone(), op, (**right).clone())
            }
            Op::Grouping(ref op) => self.evaluate(*op.clone()),
        }
    }
}

#[derive(Clone)]
pub struct InterpreterDebug {
    variables: HashMap<String, Primitive>,
}

impl InterpreterDebug {
    pub fn interpret_debug(&mut self, operations: impl Iterator<Item = Statement>) {
        let mut runtime = Interpreter {
            statements: operations.collect(),
            index: 0,
            variables: self.variables.clone(),
        };

        loop {
            println!(
                "=> {}",
                runtime.statement(runtime.statements[runtime.index].clone())
            );
            if !runtime.next() {
                break;
            }
        }

        self.variables = runtime.variables
    }
}

impl Default for InterpreterDebug {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}
