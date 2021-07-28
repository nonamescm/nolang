use super::{Env, Interpreter, Primitive};
use crate::frontend::Statement;
use std::{collections::HashMap, rc::Rc};

/// Wrapper interpreter function
pub fn interpret(operations: impl Iterator<Item = Statement>, vars: Option<Env>) -> Primitive {
    let mut runtime = match vars {
        Some(v) => Interpreter {
            statements: operations.collect(),
            index: 0,
            variables: Env::new(HashMap::new(), Some(Rc::new(v))),
        },
        None => Interpreter {
            statements: operations.collect(),
            index: 0,
            variables: Env::new(HashMap::new(), None),
        },
    };

    loop {
        let current = runtime.statement(runtime.statements[runtime.index].clone());
        if !runtime.next() {
            break current;
        }
    }
}
