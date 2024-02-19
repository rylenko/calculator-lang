pub trait Interpret {
	fn interpret(&self) -> crate::grammar::Float;
}

#[cfg(test)]
mod tests {
	use {
		super::Interpret as _,
		anyhow::{Context as _, Result},
	};

	macro_rules! assert_interpret {
		($input:expr, $output:expr) => {
			assert_eq!(
				$output,
				Program::from_str($input)
					.with_context(|| format!(
						"Failed to build a program from {}.",
						$input
					))?
					.interpret(),
			);
		};
	}

	#[test]
	fn test_interpret() -> Result<()> {
		use {crate::grammar::Program, std::str::FromStr as _};

		assert_interpret!("-5/-1.0", 5.0);
		assert_interpret!("-(53/2) * 2", -53.0);
		assert_interpret!("2 - (2 + 2 * 2.3)", -4.6);
		assert_interpret!("2 + -(10 - 1)", -7.0);
		assert_interpret!("-(15 + (3 - (2 + 5 / 5) + 1))", -16.0);
		Ok(())
	}
}
