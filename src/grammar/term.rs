use anyhow::{bail, ensure, Context as _, Error, Result};

pub(super) enum Term {
	Float(super::float::Float),
	BracketExpression(super::bracket_expression::BracketExpression),
}

impl crate::compile::interpret::Interpret for Term {
	#[must_use]
	fn interpret(&self) -> super::float::Float {
		match self {
			Self::Float(f) => f.interpret(),
			Self::BracketExpression(be) => be.interpret(),
		}
	}
}

impl crate::compile::jit::Jit for Term {
	#[must_use]
	fn jit<'a>(
		&self,
		float_type: inkwell::types::FloatType<'a>,
		builder: &'a inkwell::builder::Builder<'a>,
	) -> inkwell::values::FloatValue<'a> {
		match self {
			Self::Float(f) => f.jit(float_type, builder),
			Self::BracketExpression(be) => be.jit(float_type, builder),
		}
	}
}

impl crate::compile::vm::Instruct for Term {
	fn instruct(&self, bytecode: &mut crate::compile::vm::Bytecode) {
		match self {
			Self::Float(f) => f.instruct(bytecode),
			Self::BracketExpression(be) => be.instruct(bytecode),
		}
	}
}

impl std::convert::TryFrom<super::parser::Pair<'_>> for Term {
	type Error = Error;

	fn try_from(pair: super::parser::Pair) -> Result<Self> {
		ensure!(
			pair.as_rule() == super::parser::Rule::Term,
			"Current rule isn't a term.",
		);

		let inner = pair.into_inner().next().context("No inner.")?;
		match inner.as_rule() {
			super::parser::Rule::Float => {
				// `TryFrom<Pair>` cannot be implemented for `f64`
				let float = inner
					.as_str()
					.parse()
					.context("Failed to parse an integer.")?;
				Ok(Self::Float(float))
			}
			super::parser::Rule::BracketExpression => {
				let be =
					super::bracket_expression::BracketExpression::try_from(
						inner,
					)
					.context("Failed to build a bracket expression.")?;
				Ok(Self::BracketExpression(be))
			}
			_ => bail!("Invalid rule."),
		}
	}
}
