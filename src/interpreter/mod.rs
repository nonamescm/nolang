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

#[derive(Debug, Clone)]
pub struct Env<'a> {
    current: HashMap<String, Primitive>,
    over: Option<&'a Env<'a>>,
}

impl<'a> Default for Env<'a> {
    fn default() -> Self {
        let mut current = HashMap::new();
        current.insert(
            "writeln".to_string(),
            Primitive::NativeFunc(|arg: Primitive| {
                println!("{}", arg);
                stdout().flush().expect("Error writing line");
                Primitive::None
            }),
        );

        current.insert(
            "write".to_string(),
            Primitive::NativeFunc(|arg: Primitive| {
                println!("{}", arg);
                stdout().flush().expect("Error writing line");
                Primitive::None
            }),
        );

        current.insert(
            "__print_typeof".to_string(),
            Primitive::NativeFunc(|arg: Primitive| {
                println!("{:?}", arg);
                Primitive::None
            }),
        );

        current.insert(
            "typeof".to_string(),
            Primitive::NativeFunc(|arg: Primitive| {
                Primitive::Str(
                    match arg {
                        Primitive::NativeFunc(..) | Primitive::Function(..) => "Function",
                        Primitive::None => "None",
                        Primitive::Int(..) => "Int",
                        Primitive::BigInt(..) => "BigInt",
                        Primitive::Float(..) => "Float",
                        Primitive::Bool(..) => "Bool",
                        Primitive::Str(..) => "Str",
                    }
                    .to_string(),
                )
            }),
        );

        Self {
            current,
            over: None,
        }
    }
}

impl<'a> Env<'a> {
    fn new(current: HashMap<String, Primitive>, over: Option<&'a Env<'a>>) -> Self {
        Self { current, over }
    }

    fn set(&mut self, name: String, value: Primitive) {
        if self.current.get(&name).is_none() {
            self.current.insert(name, value);
        } else {
            crate::error!("TypeError"; "tried to reassign variable `{}`", name => 1)
        }
    }

    fn get(&self, name: &str) -> Primitive {
        match self.current.get(name) {
            Some(p) => p.clone(),
            None => match &self.over {
                Some(o) => o.get(name),
                None => {
                    crate::error!("ReferenceError"; "acessing undefined variable {}", name => 1)
                }
            },
        }
    }
}

/// The Interpreter implementation
struct Interpreter<'a> {
    statements: Vec<Statement>,
    index: usize,
    variables: Env<'a>,
}

impl<'a> Interpreter<'a> {
    /// Advance the index by one
    fn next(&mut self) -> bool {
        self.index += 1;

        self.index < self.statements.len()
    }

    /// check what's the current statement and send it for the correct evaluator
    fn statement(&mut self, statement: Statement) -> Primitive {
        match statement {
            Statement::Op(op) => self.evaluate(&op),
            Statement::Assign(var, value) => self.s_eval_assign(var, *value),
            Statement::Block(statements) => self.s_eval_block(statements),
            Statement::If(condition, block, else_block) => {
                self.s_eval_if(&condition, *block, *else_block)
            }
            Statement::FuncAssign(name, arguments, block) => {
                self.s_eval_func_assign(name, arguments, *block)
            }
            #[allow(unreachable_patterns)]
            _ => unimplemented!(), // for when I implement new statements and want to test them on the parser
        }
    }

    fn s_eval_func_assign(
        &mut self,
        name: String,
        arguments: Vec<String>,
        block: Statement,
    ) -> Primitive {
        self.variables
            .set(name, Primitive::Function(block, arguments));

        Primitive::None
    }

    /// evaluator for the block `do <Statement>;* done`
    fn s_eval_block(&mut self, statements: Vec<Statement>) -> Primitive {
        interpret(statements.into_iter(), Some(&self.variables))
    }

    fn s_eval_if(&mut self, condition: &Op, block: Statement, else_block: Statement) -> Primitive {
        if self.evaluate(condition).to_bool() {
            self.statement(block)
        } else {
            self.statement(else_block)
        }
    }

    /// Assignment `let x = <OP>;` evaluator
    fn s_eval_assign(&mut self, var: String, value: Statement) -> Primitive {
        let value = self.statement(value);

        self.variables.set(var, value);
        Primitive::None
    }

    /// Eval primary expressions, that are just the minimal possible expression
    fn eval_primary(&mut self, prim: &Literal) -> Primitive {
        match prim {
            Literal::Bool(b) => Primitive::Bool(*b),
            Literal::None => Primitive::None,
            Literal::String(ref s) => Primitive::Str(s.to_string()),
            Literal::Operation(ref op) => self.evaluate(op),
            Literal::Float(n) => Primitive::Float(*n),
            Literal::BigInt(n) => Primitive::BigInt(*n),
            Literal::Int(n) => Primitive::Int(*n),
            Literal::VarNormal(v) => self.variables.get(v),
            #[allow(unreachable_patterns)]
            _ => todo!(), // for when I add a new primary operator to the parser
        }
    }

    /// Unary expression evaluator
    fn eval_unary(&mut self, op: &Tok, right: Literal) -> Primitive {
        match op {
            Tok::Minus => -self.evaluate(&Op::Primary(Box::new(right))),
            Tok::Not => Primitive::Bool(!self.evaluate(&Op::Primary(Box::new(right)))),
            _ => unreachable!(),
        }
    }

    /// binary expression evaluator, like `1+1` or `1*1`
    fn eval_binary(&mut self, left: Op, op: &Tok, right: Op) -> Primitive {
        match op {
            // operations
            Tok::Plus => self.evaluate(&right) + self.evaluate(&left),
            Tok::Minus => self.evaluate(&right) - self.evaluate(&left),
            Tok::Asterisk => self.evaluate(&right) * self.evaluate(&left),
            Tok::Slash => self.evaluate(&right) / self.evaluate(&left),
            Tok::Percent => self.evaluate(&right) % self.evaluate(&left),

            // Comparisons
            Tok::Comp => (self.evaluate(&right) == self.evaluate(&left)).into_pri(),
            Tok::Different => (self.evaluate(&right) != self.evaluate(&left)).into_pri(),

            Tok::Gt => (self.evaluate(&right) > self.evaluate(&left)).into_pri(),
            Tok::GtOrEq => (self.evaluate(&right) >= self.evaluate(&left)).into_pri(),

            Tok::Lt => (self.evaluate(&right) < self.evaluate(&left)).into_pri(),
            Tok::LtOrEq => (self.evaluate(&right) <= self.evaluate(&left)).into_pri(),

            // Logical operators
            Tok::And => self.evaluate(&right).and(&mut || self.evaluate(&left)),
            Tok::Or => self.evaluate(&right).or(&mut || self.evaluate(&left)),

            // should not reach this since I've covered all binary operations
            _ => unreachable!(),
        }
    }

    fn eval_call(&mut self, called: &Op, arguments: Vec<Op>) -> Primitive {
        match called {
            Op::Primary(p) => match &**p {
                #[allow(unused_parens)]
                Literal::VarNormal(p) => match self.variables.get(p) {
                    Primitive::Function(block, args) => {
                        let env = args.iter().enumerate().map(|(index, key)| (
                            key.to_string(),
                            self.evaluate(arguments.get(index).unwrap_or_else(
                                || crate::error!("CallError"; "Missing arguments for function call" => 1)
                            ))
                        )).collect::<HashMap<_, _>>();

                        let env = Env::new(env, Some(&self.variables));

                        interpret(std::iter::once(block), Some(&env))
                    }
                    Primitive::NativeFunc(func) => {
                        if !matches!(arguments.len(), 1 | 0) {
                            crate::error!("CallError"; "unwrong number of arguments passed for native function" => 1);
                        }
                        func(self.evaluate(&arguments[0]))
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    /// Minimal wrapper that sends the Op to the correct evaluator
    fn evaluate(&mut self, operation: &Op) -> Primitive {
        match operation {
            Op::Primary(ref value) => self.eval_primary(&**value),
            Op::Unary(ref op, ref right) => self.eval_unary(op, *right.clone()),
            Op::Binary(ref left, ref op, ref right) => {
                self.eval_binary(*left.clone(), op, *right.clone())
            }
            Op::Grouping(ref op) => self.evaluate(op),
            Op::Call(ref called, ref arguments) => self.eval_call(&*called, arguments.clone()),

            #[allow(unreachable_patterns)]
            // for when I add a new Operation and want to test the parser before going to the
            // interpreter
            _ => unimplemented!(),
        }
    }
}
