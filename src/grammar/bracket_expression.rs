use anyhow::{ensure, Context as _, Error, Result};

pub(super) struct BracketExpression {
	is_negative: bool,
	expression: Box<super::expression::Expression>,
}

impl crate::compile::interpret::Interpret for BracketExpression {
	#[must_use]
	fn interpret(&self) -> super::float::Float {
		let value = self.expression.interpret();
		if self.is_negative {
			-value
		} else {
			value
		}
	}
}

impl crate::compile::jit::Jit for BracketExpression {
	#[must_use]
	fn jit<'a>(
		&self,
		float_type: inkwell::types::FloatType<'a>,
		builder: &'a inkwell::builder::Builder<'a>,
	) -> inkwell::values::FloatValue<'a> {
		let mut value = self.expression.jit(float_type, builder);
		if self.is_negative {
			value = value.const_neg();
		}
		value
	}
}

impl crate::compile::vm::Instruct for BracketExpression {
	fn instruct(&self, bytecode: &mut crate::compile::vm::Bytecode) {
		self.expression.instruct(bytecode);
		if self.is_negative {
			bytecode
				.add_instruction(crate::compile::vm::Instruction::UnaryMinus);
		}
	}
}

impl std::convert::TryFrom<super::parser::Pair<'_>> for BracketExpression {
	type Error = Error;

	fn try_from(pair: super::parser::Pair) -> Result<Self> {
		ensure!(
			pair.as_rule() == super::parser::Rule::BracketExpression,
			"Current rule isn't a bracket expression.",
		);

		let mut inner = pair.into_inner();
		let mut pair = inner.next().context("No inner.")?;

		// Check is negative
		let is_negative = if pair.as_rule() == super::parser::Rule::UnaryMinus
		{
			pair = inner.next().context("No expression.")?;
			true
		} else {
			false
		};

		// Get expression
		let expression = {
			ensure!(
				pair.as_rule() == super::parser::Rule::Expression,
				"Rule isn't an expression.",
			);
			let e = super::expression::Expression::try_from(pair)
				.context("Failed to build an expression.")?;
			Box::new(e)
		};

		Ok(Self { is_negative, expression })
	}
}
