use bpaf::{Bpaf, FromUtf8};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Bpaf)]
/// Solve Sudoku problems *blazingly fast*.
///
/// Accepts input from a text file to fill in the puzzle. Each number (1-9) is interpreted
/// as that number in the puzzle. A zero or any other letter is considered a blank space.
/// Any whitespace is ignored.
#[bpaf(options, version)]
pub struct Config {
    /// Location of puzzle to read or stdin by default.
    #[bpaf(short, long, argument("FILE"))]
    pub input: Option<PathBuf>,
    /// Output file to write solution to or stdout by default.
    #[bpaf(short, long, argument("FILE"))]
    pub output: Option<PathBuf>,
    /// Print puzzle with nice borders, options include `simple`, `multiline` and `bordered`
    #[bpaf(long, argument::<FromUtf8<OutputStyle>>("STYLE"), fallback(OutputStyle::Bordered))]
    pub style: OutputStyle,
    /// Print each partial solution to the console as the program runs.
    #[bpaf(short, long)]
    pub print_partials: bool,
    /// Add delay between each iteration in ms (useful when using `--print-partials`).
    #[bpaf(short, long, argument("DELAY"))]
    pub delay: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
/// How the puzzle is printed.
pub enum OutputStyle {
    /// The entire puzzle is printed in one line.
    Simple,
    /// The puzzle is printed with 9 numbers on each line.
    MultiLine,
    /// The puzzle is printed with a border around the puzzle and around each 3x3 square.
    Bordered,
}

impl FromStr for OutputStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "simple" => Ok(Self::Simple),
            "multiline" => Ok(Self::MultiLine),
            "bordered" => Ok(Self::Bordered),
            _ => Err("Invalid output style, expected simple|multiline|bordered"),
        }
    }
}
