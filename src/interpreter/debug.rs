use super::{Env, Interpreter};
use crate::frontend::Statement;

pub struct InterpreterDebug<'a> {
    variables: Env<'a>,
}

impl<'a> InterpreterDebug<'a> {
    pub fn interpret_debug(&mut self, mut operations: impl Iterator<Item = Statement>) {
        let mut runtime = Interpreter {
            variables: self.variables.clone(),
        };

        while let Some(op) = operations.next() {
            println!("=> {}", runtime.statement(op));
        }

        self.variables = runtime.variables
    }
}

impl<'a> Default for InterpreterDebug<'a> {
    fn default() -> Self {
        let variables = Env::default();

        Self { variables }
    }
}
