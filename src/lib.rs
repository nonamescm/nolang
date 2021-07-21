pub mod colors;
pub mod lexer;

#[macro_export]
macro_rules! err {
    // unexpected
    (unexpected $ch:expr, $line:expr => $val:expr) => {{
        eprintln!("unexpected token `{}` at line {}", $ch, $line);
        std::process::exit($val)
    }};

    // unclosed delimiter
    (unclosed $line:expr => $val:expr) => {{
        eprintln!("unclosed delimiter opened at line {}", $line);
        std::process::exit($val)
    }};

    // custom
    (custom $arg:expr => $val:expr) => {{
        eprintln!("{}", $arg);
        std::process::exit($val)
    }};
}
