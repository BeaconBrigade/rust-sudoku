use bpaf::{Bpaf, FromOsStr};
use std::path::PathBuf;

#[derive(Debug, Clone, Bpaf)]
/// Solve Sudoku problems *blazingly fast*.
///
/// Accepts input from a text file to fill in the puzzle. Each number (1-9) is interpreted
/// as that number in the puzzle. A zero or any other letter is considered a blank space.
/// Any whitespace is ignored.
#[bpaf(options, version)]
pub struct Config {
    /// Location of puzzle to read.
    #[bpaf(short, long, argument("FILE"))]
    pub input: Option<PathBuf>,
    /// Output file to write solution to. Leave blank to write to stdout.
    #[bpaf(short, long, argument("FILE"))]
    pub output: Option<PathBuf>,
    /// Print puzzle with nice borders, options include `simple`, `multiline` and `bordered`
    #[bpaf(long, argument("STYLE"), fallback(OutputStyle::Bordered))]
    pub style: OutputStyle,
    /// Print each partial solution to the console as the program runs.
    #[bpaf(short, long, fallback(false))]
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

impl FromOsStr for OutputStyle {
    type Out = Self;

    fn from_os_str(s: std::ffi::OsString) -> Result<Self::Out, String>
    where
        Self: Sized,
    {
        match s.to_str().ok_or_else(|| "Invalid utf8".to_string())? {
            "simple" => Ok(Self::Simple),
            "multiline" => Ok(Self::MultiLine),
            "bordered" => Ok(Self::Bordered),
            _ => Err("Invalid output style, expected simple|multiline|bordered".to_string()),
        }
    }
}
