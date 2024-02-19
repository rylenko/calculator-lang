macro_rules! fieldless_instruction_discriminants {
	($($const_name:ident => $variant:ident),* $(,)?) => {
		$(
			pub const $const_name: u8 = Self::$variant.discriminant();
		)*
	};
}

pub trait Instruct {
	fn instruct(&self, bytecode: &mut super::bytecode::Bytecode);

	#[must_use]
	fn run(&self) -> crate::grammar::Float {
		let mut bytecode = super::bytecode::Bytecode::new();
		self.instruct(&mut bytecode);

		super::Vm::new(bytecode).run()
	}
}

pub(super) type InstructionBytesRef<'a> = &'a [u8];
pub(super) type InstructionBytes = Vec<u8>;

impl From<Instruction> for InstructionBytes {
	#[must_use]
	fn from(instruction: Instruction) -> Self {
		let mut rv = vec![instruction.discriminant()];
		if let Instruction::FloatIndex(value) = instruction {
			rv.extend(value.to_be_bytes());
		}
		rv
	}
}

#[non_exhaustive]
#[repr(u8)]
pub enum Instruction {
	FloatIndex(usize),
	Add,
	Substract,
	Multiply,
	Divide,
	UnaryMinus,
}

impl Instruction {
	// For `match` statements
	pub const FLOAT_INDEX_DISCRIMINANT: u8 =
		Self::FloatIndex(0).discriminant();

	fieldless_instruction_discriminants!(
		ADD_DISCRIMINANT => Add,
		SUBSTRACT_DISCRIMINANT => Substract,
		MULTIPLY_DISCRIMINANT => Multiply,
		DIVIDE_DISCRIMINANT => Divide,
		UNARY_MINUS_DISCRIMINANT => UnaryMinus,
	);

	const fn discriminant(&self) -> u8 {
		unsafe { *(self as *const Self as *const u8) }
	}
}

impl From<crate::grammar::Operator> for Instruction {
	fn from(operator: crate::grammar::Operator) -> Self {
		match operator {
			crate::grammar::Operator::Add => Self::Add,
			crate::grammar::Operator::Substract => Self::Substract,
			crate::grammar::Operator::Multiply => Self::Multiply,
			crate::grammar::Operator::Divide => Self::Divide,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Instruction, InstructionBytes};

	#[test]
	fn test_into_instruction_bytes() {
		let instruction = Instruction::FloatIndex(12345);
		assert_eq!(
			vec![instruction.discriminant(), 0, 0, 0, 0, 0, 0, 48, 57],
			InstructionBytes::from(instruction),
		);
	}
}
