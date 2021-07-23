use crate::backend::{Literal, Op, Tokens as Tok};
use std::ops;

#[derive(Debug)]
enum Primitive {
    Number(f64),
    String(String),
    Bool(bool),
    None
}

impl ops::Not for Primitive {
    type Output = bool;

    fn not(self) -> Self::Output {
        match self {
            Self::Bool(false) => true,
            Self::None => true,
            Self::Number(x) if x == 0.0 => true,
            _ => false
        }
    }
}

impl ops::Neg for Primitive {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(n) => Primitive::Number(-n),
            Self::Bool(true) => Primitive::Number(-1f64),
            Self::Bool(false) => Primitive::Number(0f64),
            Self::String(s) => Primitive::Number(s.len() as f64),
            Self::None => Primitive::Number(0f64)
        }
    }
}

impl Primitive {
    #[allow(dead_code)]
    pub fn boolean(self) -> bool {
        !!self
    }
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
            Op::Primary(ref value) => match **value {
                Literal::Bool(b) => Primitive::Bool(b),
                Literal::None => Primitive::None,
                Literal::String(ref s) => Primitive::String(s.clone()),
                Literal::Operation(ref op) => self.evaluate(op),
                Literal::Number(n) => Primitive::Number(n),

                _ => todo!(),
            }

            Op::Unary(ref op, ref right) => match *op {
                Tok::Minus => -self.evaluate(&Op::Primary(
                    Box::new( (**right).clone() )
                )),

                Tok::Not => Primitive::Bool(!self.evaluate(
                    &Op::Primary(Box::new(
                        (**right).clone()
                    ))
                )),
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
