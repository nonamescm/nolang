#[macro_export]
macro_rules! unexpected {
    ($ch:expr, $line:expr => $val:expr) => {{
        eprintln!("unexpected token `{}` at line {}", $ch, $line);
        std::process::exit($val)
    }}
}
