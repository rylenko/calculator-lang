use anyhow::{Context as _, Result};

const PROMPT: &str = ">>> ";

pub fn run(mode: crate::cli::CompileMode) -> Result<()> {
	use {
		crate::compile::{interpret::Interpret, jit::Jit, vm::Instruct},
		std::{io::Write, str::FromStr as _},
	};
	println!("Running REPL in \"{mode}\" mode...");

	let mut line = String::new();
	let mut stdout = std::io::stdout();
	let stdin = std::io::stdin();

	loop {
		// Print the prompt
		print!("{PROMPT}");
		stdout.flush().context("Failed to flush a prompt.")?;

		// Clear current and read a new line
		line.clear();
		let bytes =
			stdin.read_line(&mut line).context("Failed to read a line.")?;

		// Check EOF
		if bytes == 0 {
			break;
		}
		if line == "exit\n" || line == "quit\n" {
			println!("Press CTRL-D (EOF).");
			continue;
		}

		// Build a program and calculate
		match crate::grammar::Program::from_str(&line) {
			Ok(p) => {
				let result = match mode {
					crate::cli::CompileMode::Interpret => p.interpret(),
					crate::cli::CompileMode::Jit => p.call(),
					crate::cli::CompileMode::Vm => p.run(),
				};
				println!("{result}");
			}
			Err(e) => println!("Failed to parse: {e:?}"),
		}
	}
	Ok(())
}
