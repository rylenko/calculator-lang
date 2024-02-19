mod cli;
mod compile;
mod grammar;
mod repl;

use anyhow::{Context as _, Result};

fn main() -> Result<()> {
	repl::run(cli::extract_compile_mode()).context("Failed to run the REPL.")
}
