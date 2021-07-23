#![allow(dead_code, unused_imports, unused_variables)]
use crate::backend::{Literal, Op, Tokens as Tok};

#[derive(Debug)]
enum Primitive<'a> {
    Number(f64),
    String(&'a String),
    Bool(bool),
    None
}

#[allow(dead_code)]
pub struct Interpreter {
    operations: Vec<Op>,
    index: usize
}

impl Interpreter {
    pub fn interpret(operations: impl Iterator<Item = Op>) {
        let mut eself = Self {
            operations: operations.collect(),
            index: 0
        };

        loop {
            println!("{:#?}", eself.evaluate(&eself.operations[eself.index]));
            if !eself.next(){ break }
        }
    }

    fn next(&mut self) -> bool {
        self.index+=1;

        self.index < self.operations.len()
    }

    /// Evaluate any Operation into an Primitive value
    fn evaluate(&self, operation: &Op) -> Primitive {
        match operation {
            Op::Unary(ref op, ref right) => match *op {
                Tok::Minus => match **right {
                    Literal::Number(n) => Primitive::Number(-n),
                    _ => todo!()
                }

                Tok::Not => Primitive::Bool(!right.boolean()),
                _ => panic!() // Unreachable
            }

            /* Needs custom implementation, but I'll commit what I've already done before
            Op::Binary(ref left, ref op, ref right) => match *op {
                Tok::Minus => match (**right, **left) {
                    (Op::Literal(n1), Op::Literal(n2)) => Primitive::Number(*n1 - *n2),
                    _ => panic!()
                }
            }
            */
            _ => todo!()
        }
    }
}
