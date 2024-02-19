#[derive(Default)]
pub struct Bytecode {
	floats: Vec<crate::grammar::Float>,
	instruction_bytes: super::instruction::InstructionBytes,
}

impl Bytecode {
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[inline]
	#[must_use]
	pub fn floats(&self) -> &[crate::grammar::Float] {
		&self.floats
	}

	#[inline]
	#[must_use]
	pub fn instruction_bytes(
		&self,
	) -> super::instruction::InstructionBytesRef {
		&self.instruction_bytes
	}

	#[inline]
	pub fn add_float(&mut self, value: crate::grammar::Float) {
		self.floats.push(value);
	}

	pub fn add_instruction(&mut self, value: super::instruction::Instruction) {
		self.instruction_bytes
			.extend(super::instruction::InstructionBytes::from(value));
	}
}

#[cfg(test)]
mod tests {
	use super::Bytecode;

	#[test]
	fn test_program() {
		use {crate::compile::vm::Instruct as _, std::str::FromStr as _};

		let mut bytecode = Bytecode::new();
		let program = crate::grammar::Program::from_str("2.1 + 1.2").unwrap();
		program.instruct(&mut bytecode);

		assert_eq!(bytecode.floats, vec![2.1, 1.2]);
		assert_eq!(
			bytecode.instruction_bytes,
			vec![
				crate::compile::vm::Instruction::FloatIndex(0),
				crate::compile::vm::Instruction::FloatIndex(1),
				crate::compile::vm::Instruction::Add,
			]
			.into_iter()
			.flat_map(super::super::instruction::InstructionBytes::from)
			.collect::<super::super::instruction::InstructionBytes>(),
		);
	}
}
