use super::Interpreter;
use super::Primitive;
use crate::frontend::Statement;
use std::collections::HashMap;

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
        let variables = HashMap::new();

        Self { variables }
    }
}
