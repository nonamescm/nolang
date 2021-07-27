#[macro_use]
mod primitive;
mod debug;
mod interpret;

pub use debug::InterpreterDebug;
pub use interpret::interpret;

use primitive::{IntoPrimitive, Primitive};
use std::collections::HashMap;
use std::io::{stdout, Write};

use crate::frontend::{Literal, Op, Statement, Tokens as Tok};

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

    /// check what's the current statement and send it for the correct evaluator
    fn statement(&mut self, statement: Statement) -> Primitive {
        match statement {
            Statement::Op(op) => self.evaluate(op),
            Statement::Write(value) => self.s_eval_write(value),
            Statement::Writeln(value) => self.s_eval_writeln(value),
            Statement::Assign(var, value) => self.s_eval_assign(var, *value),
            Statement::Block(statements) => self.s_eval_block(statements),
            Statement::If(condition, block, else_block) => self.s_eval_if(condition, *block, else_block),
            #[allow(unreachable_patterns)]
            _ => unimplemented!(), // for when I implement new statements and want to test them on the parser
        }
    }

    /// evaluator for the block `do <Statement>;* done`
    fn s_eval_block(&mut self, statements: Vec<Statement>) -> Primitive {
        interpret(statements.into_iter(), Some(self.variables.clone()))
    }

    fn s_eval_if(&mut self, condition: Op, block: Statement, else_block: Option<Box<Statement>>) -> Primitive {
        if self.evaluate(condition).to_bool() {
            self.statement(block)
        } else if else_block.is_some() {
            self.statement(*else_block.unwrap())
        } else {
            crate::error!("Syntax Error"; "expected else after if" => 1)
        }
    }

    /// `write <OP>;` statement evaluator
    fn s_eval_write(&mut self, value: Op) -> Primitive {
        print!("{}", self.evaluate(value));
        stdout().flush().unwrap();
        Primitive::None
    }

    /// `writeln <OP>;` statement evaluator
    fn s_eval_writeln(&mut self, value: Op) -> Primitive {
        println!("{}", self.evaluate(value));
        stdout().flush().unwrap();
        Primitive::None
    }

    /// Assignment `let x = <OP>;` evaluator
    fn s_eval_assign(&mut self, var: String, value: Statement) -> Primitive {
        let value = self.statement(value);

        if self.variables.get(&var).is_some() {
            crate::error!("TypeError"; "tried to reassign global constant {}", var => 1)
        }
        self.variables.insert(var, value);
        Primitive::None
    }

    /// Eval primary expressions, that are just the minimal possible expression
    #[rustfmt::skip]
    // I like some syntax on this function but rustfmt removes it
    fn eval_primary(&mut self, prim: Literal) -> Primitive {
        match prim {
            Literal::Bool(b) => Primitive::Bool(b),
            Literal::None => Primitive::None,
            Literal::String(ref s) => Primitive::Str(s.to_string()),
            Literal::Operation(ref op) => self.evaluate(op.clone()),
            Literal::Number(n) => Primitive::Number(n),
            Literal::VarNormal(v) =>
            (
                *self.variables.get(&v).unwrap_or_else(
                    || crate::error!("ReferenceError"; "acessing undefined variable {}", v => 1)
                )
            ).clone(),
            #[allow(unreachable_patterns)]
            _ => todo!(), // for when I add a new primary operator to the parser
        }
    }

    /// Unary expression evaluator
    fn eval_unary(&mut self, op: &Tok, right: Literal) -> Primitive {
        match op {
            Tok::Minus => Primitive::Number(-self.evaluate(Op::Primary(Box::new(right)))),
            Tok::Not => Primitive::Bool(!self.evaluate(Op::Primary(Box::new(right)))),
            _ => unreachable!(),
        }
    }

    /// binary expression evaluator, like `1+1` or `1*1`
    fn eval_binary(&mut self, left: Op, op: &Tok, right: Op) -> Primitive {
        match op {
            // operations
            Tok::Plus => self.evaluate(right) + self.evaluate(left),
            Tok::Minus => (self.evaluate(right) - self.evaluate(left)).into_pri(),
            Tok::Asterisk => (self.evaluate(right) * self.evaluate(left)).into_pri(),
            Tok::Slash => (self.evaluate(right) / self.evaluate(left)).into_pri(),

            // Comparisons
            Tok::Comp => (self.evaluate(right) == self.evaluate(left)).into_pri(),
            Tok::Different => (self.evaluate(right) != self.evaluate(left)).into_pri(),

            Tok::Gt => (self.evaluate(right) > self.evaluate(left)).into_pri(),
            Tok::GtOrEq => (self.evaluate(right) >= self.evaluate(left)).into_pri(),

            Tok::Lt => (self.evaluate(right) < self.evaluate(left)).into_pri(),
            Tok::LtOrEq => (self.evaluate(right) <= self.evaluate(left)).into_pri(),

            // Logical operators
            Tok::And => self.evaluate(right).and(&mut || self.evaluate(left.clone())),
            Tok::Or => self.evaluate(right).or(&mut || self.evaluate(left.clone())),

            // should not reach this since I've covered all binary operations
            _ => unreachable!(),
        }
    }

    fn eval_call(&mut self, called: Op, arguments: Vec<Op>) -> Primitive {
        match called {
            Op::Primary(p) => match *p {
                Literal::VarNormal(p) => {
                    match self.variables.get(&p) {
                        Some(v) => match v {
                            Primitive::Function(func) => func(arguments.into_iter().map(|v| self.evaluate(v)).collect()),
                            e => crate::error!("TypeError"; "can't call {}", e => 1)
                        }
                        _ => crate::error!("ReferenceError"; "tried to call undefined variable {}", p => 1)
                    }
                }
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }

    /// Minimal wrapper that sends the Op to the correct evaluator
    fn evaluate(&mut self, operation: Op) -> Primitive {
        match operation {
            Op::Primary(ref value) => self.eval_primary(*value.clone()),
            Op::Unary(ref op, ref right) => self.eval_unary(op, *right.clone()),
            Op::Binary(ref left, ref op, ref right) => {
                self.eval_binary(*left.clone(), op, *right.clone())
            }
            Op::Grouping(ref op) => self.evaluate(*op.clone()),
            Op::Call(ref called, ref arguments) => self.eval_call(*called.clone(), arguments.clone()),

            #[allow(unreachable_patterns)]
            // for when I add a new Operation and want to test the parser before going to the
            // interpreter
            _ => unimplemented!()
        }
    }
}
