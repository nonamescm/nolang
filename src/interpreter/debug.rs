use super::{Env, Interpreter};
use crate::frontend::Statement;

pub struct InterpreterDebug<'a> {
    variables: Env<'a>,
}

impl<'a> InterpreterDebug<'a> {
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

impl<'a> Default for InterpreterDebug<'a> {
    fn default() -> Self {
        let variables = Env::default();

        Self { variables }
    }
}
