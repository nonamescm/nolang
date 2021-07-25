use super::Interpreter;
use crate::backend::Statement;
use std::collections::HashMap;

/// Wrapper interpreter function
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
