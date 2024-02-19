use anyhow::{ensure, Context as _, Error, Result};

pub(super) enum Expression {
	Term(super::term::Term),
	Operation {
		left: Box<Self>,
		operator: super::operator::Operator,
		right: Box<Self>,
	},
}

impl crate::compile::interpret::Interpret for Expression {
	#[must_use]
	fn interpret(&self) -> super::float::Float {
		match self {
			Self::Term(t) => t.interpret(),
			Self::Operation { left, operator, right } => {
				operator.interpret_with(left.interpret(), right.interpret())
			}
		}
	}
}

impl crate::compile::jit::Jit for Expression {
	#[must_use]
	fn jit<'a>(
		&self,
		float_type: inkwell::types::FloatType<'a>,
		builder: &'a inkwell::builder::Builder<'a>,
	) -> inkwell::values::FloatValue<'a> {
		match self {
			Self::Term(t) => t.jit(float_type, builder),
			Self::Operation { left, operator, right } => operator.jit_with(
				float_type,
				builder,
				left.jit(float_type, builder),
				right.jit(float_type, builder),
			),
		}
	}
}

impl crate::compile::vm::Instruct for Expression {
	fn instruct(&self, bytecode: &mut crate::compile::vm::Bytecode) {
		match self {
			Self::Term(t) => t.instruct(bytecode),
			Self::Operation { left, operator, right } => {
				left.instruct(bytecode);
				right.instruct(bytecode);
				operator.instruct(bytecode);
			}
		}
	}
}

impl std::convert::TryFrom<super::parser::Pair<'_>> for Expression {
	type Error = Error;

	fn try_from(pair: super::parser::Pair) -> Result<Self> {
		ensure!(
			pair.as_rule() == super::parser::Rule::Expression,
			"Current rule isn't a varied expression.",
		);

		super::parser::PRATT
			.map_primary(|p| match p.as_rule() {
				super::parser::Rule::Term => {
					let term = super::term::Term::try_from(p)
						.context("Failed to build a term.")?;
					Ok(Self::Term(term))
				}
				_ => unreachable!("Invalid rule."),
			})
			.map_infix(|left: Result<Self>, operator, right: Result<Self>| {
				let left = Box::new(left.context("Failed to build left.")?);
				let operator = super::operator::Operator::try_from(operator)
					.context("Failed to build an operator.")?;
				let right = Box::new(right.context("Failed to build right.")?);

				Ok(Self::Operation { left, operator, right })
			})
			.parse(pair.into_inner())
			.context("Failed to parse.")
	}
}
