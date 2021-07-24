use nolang::colors::Colors;
use nolang::{
    interpreter::Interpreter,
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
        interpret()?;
    }
    Ok(())
}

fn interpret() -> IOResult<()> {
    let mut arguments = args();
    arguments.next();

    for file in arguments {
        Interpreter::interpret(parse(read_to_string(file)?));
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
        None => loop {
            let input = print_read()?;
            Interpreter::interpret(parse(input.to_string()));
        }
    }
}
