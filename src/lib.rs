#[macro_export]
macro_rules! err {
    // unexpected
    ($ch:expr, $line:expr => $val:expr) => {{
        eprintln!("unexpected token `{}` at line {}", $ch, $line);
        std::process::exit($val)
    }};

    // unclosed delimiter
    ($line:expr => $val:expr) => {{
        eprintln!("unclosed delimiter opened at line {}", $line);
        std::process::exit($val)
    }};
}
