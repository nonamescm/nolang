use nolang::{interpret_file, repl};

use std::{env::args, io};

fn main() -> io::Result<()> {
    let args = args().collect::<Vec<String>>();

    let flairs: Vec<&str> = args.iter().map(|x| x.as_str()).collect();

    if args.iter().filter(|x| !x.starts_with('-')).count() < 2 {
        repl(flairs.as_slice())?;
    } else {
        interpret_file()?;
    }
    Ok(())
}
