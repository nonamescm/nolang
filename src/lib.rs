pub mod frontend;
pub mod interpreter;

mod util;

pub use util::{colors::Colors, interpret_file::interpret_file, repl::repl};

#[macro_export]
macro_rules! error {
    ($error_type:expr; $($format_args:expr),+ => $exit_value: expr) => {{
        eprintln!(
            "\x1b[1m{}\x1b[0m",
            $crate::Colors::colorize(
                $crate::Colors::Red,
                &format!("├ {}:", $error_type)
            )
        );
        eprint!(
            "\x1b[1m{}\x1b[0m",
            $crate::Colors::colorize(
                $crate::Colors::Red,
                "└─ "
            )
        );
        eprintln!($($format_args),+);

        std::panic::set_hook(Box::new(|_| {}));
        panic!()
    }}
}
