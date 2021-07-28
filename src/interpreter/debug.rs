use super::{Env, Interpreter};
use crate::frontend::Statement;

pub struct InterpreterDebug {
    variables: Env,
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
        let variables = Env::default();

        Self { variables }
    }
}
