use crate::{frontend::parse, interpreter::interpret};
use std::{env::args, fs::read_to_string, io};

pub fn interpret_file() -> io::Result<()> {
    let mut arguments = args();
    arguments.next();

    for file in arguments {
        interpret(parse(read_to_string(file)?), None);
    }
    Ok(())
}
