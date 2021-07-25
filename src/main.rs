use nolang::colors::Colors;
use std::sync::Mutex;
use nolang::{
    interpreter::{
        interpret,
        InterpreterDebug
    },
    backend::{lex, parse}
};

use std::{
    env::{args, var},
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

type IOResult<T> = Result<T, std::io::Error>;

fn main() -> IOResult<()> {
    let flairs = std::env::args().collect::<Vec<_>>();
    let flairs: Vec<&str> = flairs.iter().map(|x| &**x).collect();


    if args().filter(|x| !x.starts_with('-')).count() < 2 {
        repl(flairs.as_slice())?;
    } else {
        read_file()?;
    }
    Ok(())
}

fn read_file() -> IOResult<()> {
    let mut arguments = args();
    arguments.next();

    for file in arguments {
        interpret(parse(read_to_string(file)?));
    }
    Ok(())
}

fn print_read() -> IOResult<String> {
    use Colors::*;

    print!(
        "{}({}){} ",
        Colors::colorize(Purple, &var("USER").unwrap_or_else(|_| "REPL".to_string())),
        Colors::colorize(LightBlue, "NoLang"),
        Colors::colorize(Green, ">")
    );

    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}

fn repl(arguments: &[&str]) -> IOResult<()> {
    match arguments.get(1) {
        Some(&"-p") => loop {
            let input = print_read()?;
            println!("{:#?}", parse(input).collect::<Vec<_>>());
        }
        Some(&"-l") => loop {
            let input = print_read()?;
            println!("{:#?}", lex(input).collect::<Vec<_>>())
        }
        Some(e) => panic!("Unrecognized option `{}`", e),
        None => {
            loop {
                let runtime_debug = Mutex::new(
                    InterpreterDebug::default()
                );

                loop {
                    let input = print_read()?;
                    std::panic::catch_unwind(|| {
                        runtime_debug.lock().unwrap()
                            .interpret_debug(
                                parse(input.to_string())
                            );
                    }).unwrap_or_default();
                    if runtime_debug.is_poisoned() { break }
                }
            }
        }
    }
}
