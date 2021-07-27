use super::{Interpreter, Primitive};
use crate::frontend::Statement;
use std::collections::HashMap;

/// Wrapper interpreter function
pub fn interpret(
    operations: impl Iterator<Item = Statement>,
    vars: Option<HashMap<String, Primitive>>,
) -> Primitive {
    let mut runtime = Interpreter {
        statements: operations.collect(),
        index: 0,
        variables: vars.unwrap_or_default(),
    };

    loop {
        let current = runtime.statement(runtime.statements[runtime.index].clone());
        if !runtime.next() {
            break current;
        }
    }
}
