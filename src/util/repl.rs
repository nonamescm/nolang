use std::{
    env::var,
    io::{self, stdin, stdout, Write},
    panic::catch_unwind,
    sync::Mutex,
};

use crate::{
    backend::{lex, parse},
    interpreter::InterpreterDebug,
    util::colors::Colors,
};

fn print_read() -> io::Result<String> {
    use Colors::*;

    print!(
        "{}({}){} ",
        Colors::colorize(Purple, &var("USER").unwrap_or_else(|_| "REPL".to_string())),
        Colors::colorize(LightBlue, "nolang"),
        Colors::colorize(Green, ">")
    );

    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn repl(arguments: &[&str]) -> io::Result<()> {
    match arguments.get(1) {
        Some(&"-p") => loop {
            let input = print_read()?;
            println!("{:#?}", parse(input).collect::<Vec<_>>());
        },
        Some(&"-l") => loop {
            let input = print_read()?;
            println!("{:#?}", lex(input).collect::<Vec<_>>())
        },
        Some(e) => panic!("Unrecognized option `{}`", e),
        None => loop {
            let runtime = Mutex::new(InterpreterDebug::default());

            while !runtime.is_poisoned() {
                let input = print_read()?;

                catch_unwind(|| {
                    runtime
                        .lock()
                        .unwrap()
                        .interpret_debug(parse(input.to_string()))
                })
                .unwrap_or_default();
            }
        },
    }
}
