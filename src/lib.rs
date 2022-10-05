//! # Rust sudoku solver
//!
//! Simple cli program to solve sudoku puzzles.
//!
//! ## Usage
//!
//! The program reads a puzzle reads a puzzle from a file or stdin using the
//! following format:
//!
//! The puzzle is filled in left-right top-bottom where every number 1-9 is
//! put into the current place in the puzzle, 0 or any other letter is considered
//! a blank space, and any whitespace is ignored.

/// The configuration of the project gotten from CLI.
pub mod cli;

use crate::cli::{Config, OutputStyle};
use anyhow::{anyhow, Context};
use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::PathBuf,
};

/// Sudoku solver. Takes in a config and will solve the puzzle.
#[derive(Debug, Clone)]
pub struct SudokuSolver {
    config: Config,
    root: Node,
    solution: Option<Box<Node>>,
}

impl SudokuSolver {
    /// Create solution from [`Config`].
    pub fn new(config: Config) -> anyhow::Result<Self> {
        let starting_point = parse(&config.input)?;
        Ok(Self {
            config,
            root: Node {
                candidate: starting_point,
                ..Default::default()
            },
            solution: None,
        })
    }

    /// Solves the puzzle and writes output to file specified in config.
    pub fn solve(&mut self) -> anyhow::Result<()> {
        self.solution = self.root.backtrack(&self.config);

        self.output()?;

        Ok(())
    }

    fn output(&mut self) -> anyhow::Result<()> {
        let solution = self
            .solution
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't solve puzzle."))?;

        solution.print_solution(&self.config)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Node {
    /// This node's candidate puzzle.
    pub candidate: [u8; 81],
    /// Index of most recent child.
    pub most_recent: usize,
    /// Descendants of this node. The progression of the node.
    pub children: [Option<Box<Node>>; 9],
}

impl Default for Node {
    fn default() -> Self {
        Self {
            candidate: [0; 81],
            // children: [None, None, None, None, None, None, None, None, None],
            most_recent: 0,
            children: Default::default(),
        }
    }
}

impl Node {
    /// Recursively solve the puzzle.
    fn backtrack(&mut self, config: &Config) -> Option<Box<Node>> {
        // Print the partial solutions and add delay
        if let Some(del) = config.delay {
            std::thread::sleep(std::time::Duration::from_millis(del));
        }
        if config.print_partials {
            self.print_solution(config).unwrap();
        }

        // Base cases
        if self.reject() {
            return None;
        } else if self.accept() {
            return Some(Box::new(self.clone()));
        }

        // recursively solve the puzzle
        let to_change = self.first();
        let mut next = self.children[self.most_recent].as_mut();

        // create next 8 children
        while let Some(child) = next {
            if let Some(solution) = child.backtrack(config) {
                return Some(solution);
            }
            if self.next(to_change).is_some() {
                next = self.children[self.most_recent].as_mut();
            } else {
                break;
            }
        }

        None
    }

    /// Generate first child of parent.
    fn first(&mut self) -> usize {
        let mut child = Node {
            candidate: self.candidate,
            ..Default::default()
        };
        let to_change = child
            .candidate
            .iter()
            .position(|&x| x == 0)
            .expect("Already checked that puzzle isn't complete in `backtrack`");

        child.candidate[to_change] = 1;

        // self.most_recent += 1;
        self.children[self.most_recent] = Some(Box::new(child));

        to_change
    }

    /// Generate next child after prev.
    fn next(&mut self, to_change: usize) -> Option<()> {
        if self.most_recent >= 8 {
            return None;
        }
        let prev = &mut self.children[self.most_recent]
            .as_mut()
            .expect("Known to be valid since we're calling this next");

        let mut child = Node {
            candidate: prev.candidate,
            ..Default::default()
        };
        child.candidate[to_change] += 1;

        self.most_recent += 1;
        self.children[self.most_recent] = Some(Box::new(child));

        Some(())
    }

    /// Check if everysquare is full.
    fn accept(&self) -> bool {
        !self.candidate.contains(&0)
    }

    /// Check if candidate is still valid.
    fn reject(&self) -> bool {
        let mut counter = [0u8; 10]; // first element is for blanks
        let candidate = &self.candidate;

        // Check horizontal rows
        for i in 0..9 {
            for j in 0..9 {
                counter[candidate[(i * 9 + j) as usize] as usize] += 1;
            }

            for j in counter.iter_mut().skip(1) {
                if *j > 1 {
                    return true;
                }

                *j = 0;
            }
        }

        // check vertical rows
        for i in 0..9 {
            for j in 0..9 {
                counter[candidate[(j * 9 + i) as usize] as usize] += 1;
            }

            for j in counter.iter_mut().skip(1) {
                if *j > 1 {
                    return true;
                }

                *j = 0;
            }
        }

        // check squares
        // traverse rows
        for i in 0..3 {
            // traverse columns
            for j in 0..3 {
                let offset: usize = (i * 27) + (j * 3);
                // traverse row in square
                for k in 0..3 {
                    counter[candidate[offset + k * 9] as usize] += 1;
                    counter[candidate[offset + k * 9 + 1] as usize] += 1;
                    counter[candidate[offset + k * 9 + 2] as usize] += 1;
                }

                for k in counter.iter_mut().skip(1) {
                    if *k > 1 {
                        return true;
                    }

                    *k = 0;
                }
            }
        }

        false
    }

    /// Output puzzle.
    fn print_solution(&self, config: &Config) -> anyhow::Result<()> {
        let puzzle = &self.candidate;
        let mut output = match &config.output {
            Some(path) => Box::new(File::create(path).context("Could not open output file.")?)
                as Box<dyn Write>,
            None => Box::new(io::stdout()) as Box<dyn Write>,
        };
        match config.style {
            OutputStyle::Bordered => {
                for i in 0..3 {
                    let mut offset = i * 27;
                    output.write_all(b"+-------+-------+-------+\n")?;
                    for _ in 0..3 {
                        output.write_all(
                            format!(
                                "| {} {} {} | {} {} {} | {} {} {} |\n",
                                puzzle[offset],
                                puzzle[offset + 1],
                                puzzle[offset + 2],
                                puzzle[offset + 3],
                                puzzle[offset + 4],
                                puzzle[offset + 5],
                                puzzle[offset + 6],
                                puzzle[offset + 7],
                                puzzle[offset + 8]
                            )
                            .as_bytes(),
                        )?;
                        offset += 9;
                    }
                }
                output.write_all(b"+-------+-------+-------+\n")?;
            }
            OutputStyle::MultiLine => {
                for i in 0..9 {
                    let offset = i * 9;
                    output.write_all(
                        format!(
                            "{} {} {} {} {} {} {} {} {}",
                            puzzle[offset],
                            puzzle[offset + 1],
                            puzzle[offset + 2],
                            puzzle[offset + 3],
                            puzzle[offset + 4],
                            puzzle[offset + 5],
                            puzzle[offset + 6],
                            puzzle[offset + 7],
                            puzzle[offset + 8]
                        )
                        .as_bytes(),
                    )?;
                }
                output.write_all(b"\n")?;
            }
            OutputStyle::Simple => {
                for i in puzzle {
                    output.write_all(format!("{}", i).as_bytes())?;
                }
                output.write_all(b"\n")?;
            }
        }
        Ok(())
    }
}

/// Read input from file.
fn parse(path: &Option<PathBuf>) -> anyhow::Result<[u8; 81]> {
    let contents = match path {
        Some(path) => std::fs::read_to_string(path).context("Could not read input file.")?,
        None => {
            let mut buf = String::new();
            for line in io::stdin().lock().lines() {
                buf.push_str(line?.as_str())
            }
            buf
        }
    };

    let mut i = 0;
    let mut puzzle = [0; 81];
    for c in contents.chars() {
        if c.is_whitespace() {
            continue;
        }
        let this_square = c.to_digit(10).unwrap_or(0);
        puzzle[i] = this_square as u8;
        i += 1;
        if i == 81 {
            break;
        }
    }

    if i < 81 {
        Err(anyhow!(
            "Not enough input. The file was not long enough to construct a complete puzzle."
        ))
    } else {
        Ok(puzzle)
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::OutputStyle;

    use super::*;

    #[test]
    fn sudoku1() {
        let config = Config {
            input: Some(PathBuf::from("puzzle/sudoku1.txt")),
            output: None,
            style: OutputStyle::Bordered,
            print_partials: false,
            delay: None,
        };
        let mut solution = SudokuSolver::new(config).unwrap();
        solution.solve().unwrap();

        let confirmed_solution: [u8; 81] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 5, 6, 7, 8, 9, 1, 2, 3, 7, 8, 9, 1, 2, 3, 4, 5, 6, 2, 1,
            4, 3, 6, 5, 8, 9, 7, 3, 6, 5, 8, 9, 7, 2, 1, 4, 8, 9, 7, 2, 1, 4, 3, 6, 5, 5, 3, 1, 6,
            4, 2, 9, 7, 8, 6, 4, 2, 9, 7, 8, 5, 3, 1, 9, 7, 8, 5, 3, 1, 6, 4, 2,
        ];
        assert_eq!(solution.solution.unwrap().candidate, confirmed_solution);
    }

    #[test]
    #[should_panic]
    fn not_enough_input() {
        let config = Config {
            input: Some(PathBuf::from("puzzle/sudoku8.txt")),
            output: None,
            print_partials: false,
            style: OutputStyle::Bordered,
            delay: None,
        };
        let mut solution = SudokuSolver::new(config).unwrap();
        solution.solve().unwrap();
    }
}
