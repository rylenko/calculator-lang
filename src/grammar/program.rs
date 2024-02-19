use anyhow::{ensure, Context as _, Error, Result};

pub struct Program(super::expression::Expression);

impl crate::compile::interpret::Interpret for Program {
	#[inline]
	#[must_use]
	fn interpret(&self) -> super::float::Float {
		self.0.interpret()
	}
}

impl crate::compile::jit::Jit for Program {
	#[inline]
	#[must_use]
	fn jit<'a>(
		&self,
		float_type: inkwell::types::FloatType<'a>,
		builder: &'a inkwell::builder::Builder<'a>,
	) -> inkwell::values::FloatValue<'a> {
		self.0.jit(float_type, builder)
	}
}

impl crate::compile::vm::Instruct for Program {
	#[inline]
	fn instruct(&self, bytecode: &mut crate::compile::vm::Bytecode) {
		self.0.instruct(bytecode)
	}
}

impl std::str::FromStr for Program {
	type Err = Error;

	fn from_str(source: &str) -> Result<Program> {
		use pest::Parser as _;

		let pairs =
			super::parser::Parser::parse(super::parser::Rule::Program, source)
				.context("Failed to parse a source.")?;
		for pair in pairs {
			if pair.as_rule() == super::parser::Rule::Program {
				return Program::try_from(pair)
					.context("Failed to build a program.");
			}
		}

		panic!("There is no program's rule.");
	}
}

impl std::convert::TryFrom<super::parser::Pair<'_>> for Program {
	type Error = Error;

	fn try_from(pair: super::parser::Pair) -> Result<Self> {
		ensure!(
			pair.as_rule() == super::parser::Rule::Program,
			"Current rule isn't a program.",
		);

		let pair = pair.into_inner().next().context("No expression.")?;
		let expression = super::expression::Expression::try_from(pair)
			.context("Failed to build an expression.")?;
		Ok(Self(expression))
	}
}

#[cfg(test)]
mod tests {
	use {super::Program, anyhow::Result};

	#[test]
	fn test_from_str() -> Result<()> {
		use std::str::FromStr as _;

		assert!(Program::from_str("-01").is_err());
		assert!(Program::from_str("-5+").is_err());
		assert!(Program::from_str("-5*1").is_ok());
		assert!(Program::from_str("-(5 / 1").is_err());
		assert!(Program::from_str("-5) / 5").is_err());
		assert!(Program::from_str("-5+1*3").is_ok());
		assert!(Program::from_str("-(5+1) * 2").is_ok());
		assert!(Program::from_str("+(5+1)").is_err());
		assert!(Program::from_str("+-(5 + 1)").is_err());
		assert!(Program::from_str("+(-(5 + 1)").is_err());
		assert!(Program::from_str("((5 + 1)").is_err());
		assert!(Program::from_str("(2 + (5 + 1) - 5) / 2").is_ok());
		Ok(())
	}
}
