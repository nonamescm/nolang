use nolang::colors::Colors;
use nolang::backend::parse;
use nolang::interpreter::Interpreter;
use std::{
    env::{args, var},
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

type IOResult = Result<(), std::io::Error>;

fn main() -> IOResult {
    if args().count() < 2 {
        repl()?;
    } else {
        interpret()?;
    }
    Ok(())
}

fn interpret() -> IOResult {
    let mut arguments = args();
    arguments.next();

    for file in arguments {
        let tokens = parse(read_to_string(file)?);

        println!("{:#?}", tokens.collect::<Vec<_>>())
    }
    Ok(())
}

fn repl() -> IOResult {
    use Colors::*;
    loop {
        print!(
            "{}({}){} ",
            Colors::colorize(Purple, &var("USER").unwrap_or_else(|_| "REPL".to_string())),
            Colors::colorize(LightBlue, "NoLang"),
            Colors::colorize(Green, ">")
        );

        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        /* let tokens = */Interpreter::interpret(parse(input));

        // println!("{:#?}", tokens.collect::<Vec<_>>())
    }
}
