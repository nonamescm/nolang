use std::{
    io,
    env::args,
    fs::read_to_string
};
use crate::{
    interpreter::interpret,
    backend::parse
};

pub fn interpret_file() -> io::Result<()> {
    let mut arguments = args();
    arguments.next();

    for file in arguments {
        interpret(parse(
            read_to_string(file)?
        ));
    }
    Ok(())
}
