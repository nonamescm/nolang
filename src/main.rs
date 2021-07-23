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
    let files = args().filter(|x| !x.starts_with('-')).collect::<Vec<String>>();
    let flairs = args().filter(|x| x.starts_with('-')).collect::<Vec<String>>();

    if files.len() < 2 {
        repl(flairs)?;
    } else {
        interpret()?;
    }
    Ok(())
}

fn interpret() -> IOResult<()> {
    let mut arguments = args();
    arguments.next();

    for file in arguments {
        let tokens = parse(read_to_string(file)?);

        println!("{:#?}", tokens.collect::<Vec<_>>())
    }
    Ok(())
}

fn repl(arguments: Vec<String>) -> IOResult<()> {
    use Colors::*;
    let print_read = || -> IOResult<String> {
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
    };

    if arguments.contains(&"-p".to_string()) {
        loop {
            let input = print_read()?;
            println!("{:#?}", parse(input).collect::<Vec<_>>());
        }
    } else if arguments.contains(&"-l".to_string()) {
        loop {
            let input = print_read()?;
            println!("{:#?}", lex(input).collect::<Vec<_>>())
        }
    } else {
        loop {
            let input = print_read()?;
            Interpreter::interpret(parse(input))
        }
    }
}
