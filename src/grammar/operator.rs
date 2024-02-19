use anyhow::{bail, Error, Result};

#[derive(Clone, Copy)]
pub enum Operator {
	Add,
	Substract,
	Multiply,
	Divide,
}

impl Operator {
	#[must_use]
	pub(super) fn interpret_with(
		&self,
		x: super::float::Float,
		y: super::float::Float,
	) -> super::float::Float {
		match self {
			Self::Add => x + y,
			Self::Substract => x - y,
			Self::Multiply => x * y,
			Self::Divide => x / y,
		}
	}

	#[must_use]
	pub(super) fn jit_with<'a>(
		&self,
		_int_type: inkwell::types::FloatType,
		builder: &inkwell::builder::Builder<'a>,
		x: inkwell::values::FloatValue<'a>,
		y: inkwell::values::FloatValue<'a>,
	) -> inkwell::values::FloatValue<'a> {
		match self {
			Self::Add => builder.build_float_add(x, y, "add"),
			Self::Substract => builder.build_float_sub(x, y, "sub"),
			Self::Multiply => builder.build_float_mul(x, y, "mul"),
			Self::Divide => builder.build_float_div(x, y, "div"),
		}
	}
}

impl crate::compile::interpret::Interpret for Operator {
	fn interpret(&self) -> super::float::Float {
		unimplemented!("Use `interpret_with`.");
	}
}

impl crate::compile::jit::Jit for Operator {
	fn jit<'a>(
		&self,
		_int_type: inkwell::types::FloatType,
		_builder: &inkwell::builder::Builder<'a>,
	) -> inkwell::values::FloatValue<'a> {
		unimplemented!("Use `jit_with`.");
	}
}

impl crate::compile::vm::Instruct for Operator {
	fn instruct(&self, bytecode: &mut crate::compile::vm::Bytecode) {
		bytecode.add_instruction((*self).into());
	}
}

impl std::convert::TryFrom<super::parser::Pair<'_>> for Operator {
	type Error = Error;

	fn try_from(pair: super::parser::Pair) -> Result<Self> {
		// No `anyhow::ensure` here because `Operator` is silent rule
		match pair.as_rule() {
			super::parser::Rule::Add => Ok(Self::Add),
			super::parser::Rule::Substract => Ok(Self::Substract),
			super::parser::Rule::Multiply => Ok(Self::Multiply),
			super::parser::Rule::Divide => Ok(Self::Divide),
			_ => bail!("Invalid rule."),
		}
	}
}
