use rust_sudoku::{cli, SudokuSolver};

fn main() -> anyhow::Result<()> {
    let config: cli::Config = cli::config().run();

    let mut solution = SudokuSolver::new(config)?;
    solution.solve()?;

    Ok(())
}
