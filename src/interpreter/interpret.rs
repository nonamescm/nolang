use super::{Env, Interpreter, Primitive};
use crate::frontend::Statement;
use std::collections::HashMap;

/// Wrapper interpreter function
pub fn interpret(mut operations: impl Iterator<Item = Statement>, vars: Option<&Env>) -> Primitive {
    let mut runtime = match vars {
        Some(v) => Interpreter {
            variables: Env::new(HashMap::new(), Some(v)),
        },
        None => Interpreter {
            variables: Env::default(),
        },
    };

    let mut current = Primitive::None;

    while let Some(op) = operations.next() {
        current = runtime.statement(op);
    }
    current
}
