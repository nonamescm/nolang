use nolang::{interpret_file, repl};

use std::{env::args, io};

fn main() -> io::Result<()> {
    let flairs = std::env::args().collect::<Vec<_>>();
    let flairs: Vec<&str> = flairs.iter().map(|x| &**x).collect();

    if args().filter(|x| !x.starts_with('-')).count() < 2 {
        repl(flairs.as_slice())?;
    } else {
        interpret_file()?;
    }
    Ok(())
}
