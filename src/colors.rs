#[allow(dead_code)]
pub enum Colors {
    Yellow,
    Red,
    Green,
    Blue,
    LightBlue,
    Purple,
    White,
    Black,
}

impl Colors {
    pub fn colorize(color: Colors, args: &str) -> String {
        let color_code = match color {
            Self::Black => "30",
            Self::Red => "31",
            Self::Green => "32",
            Self::Yellow => "33",
            Self::Blue => "34",
            Self::Purple => "35",
            Self::LightBlue => "36",
            Self::White => "37",
        };

        format!("\x1b[{}m{}\x1b[0m", color_code, args)
    }
}
