mod bytecode;
mod instruction;

pub use {
	bytecode::Bytecode,
	instruction::{Instruct, Instruction},
};

const STACK_LEN: usize = 128;

pub struct Vm {
	bytecode: bytecode::Bytecode,
	stack: [crate::grammar::Float; STACK_LEN],
	stack_free_index: usize,
}

impl Vm {
	#[must_use]
	pub fn new(bytecode: bytecode::Bytecode) -> Self {
		Self { bytecode, stack: [0.0; STACK_LEN], stack_free_index: 0 }
	}

	pub fn run(&mut self) -> crate::grammar::Float {
		// instruction byte index
		let mut ibi = 0;

		while ibi < self.bytecode.instruction_bytes().len() {
			ibi += 1;

			match self.bytecode.instruction_bytes()[ibi - 1] {
				instruction::Instruction::FLOAT_INDEX_DISCRIMINANT => {
					let ib = self.bytecode.instruction_bytes();
					let float_index = usize::from_be_bytes([
						ib[ibi],
						ib[ibi + 1],
						ib[ibi + 2],
						ib[ibi + 3],
						ib[ibi + 4],
						ib[ibi + 5],
						ib[ibi + 6],
						ib[ibi + 7],
					]);
					self.push(self.bytecode.floats()[float_index]);
					ibi += 8;
				}
				instruction::Instruction::ADD_DISCRIMINANT => {
					let result = self.pop() + self.pop();
					self.push(result);
				}
				instruction::Instruction::SUBSTRACT_DISCRIMINANT => {
					let right = self.pop();
					let left = self.pop();
					self.push(left - right);
				}
				instruction::Instruction::MULTIPLY_DISCRIMINANT => {
					let result = self.pop() * self.pop();
					self.push(result);
				}
				instruction::Instruction::DIVIDE_DISCRIMINANT => {
					let right = self.pop();
					let left = self.pop();
					self.push(left / right);
				}
				instruction::Instruction::UNARY_MINUS_DISCRIMINANT => {
					let value = self.pop();
					self.push(value);
				}
				_ => unimplemented!(),
			}
		}
		self.stack[0]
	}

	fn pop(&mut self) -> crate::grammar::Float {
		assert_ne!(self.stack_free_index, 0);

		let rv = self.stack[self.stack_free_index - 1];
		self.stack_free_index -= 1;
		rv
	}

	fn push(&mut self, value: crate::grammar::Float) {
		assert!(self.stack_free_index < STACK_LEN);

		self.stack[self.stack_free_index] = value;
		self.stack_free_index += 1;
	}
}

#[cfg(test)]
mod tests {
	use anyhow::{Context as _, Result};

	macro_rules! assert_run {
		($source:expr, $result:expr) => {
			let program = crate::grammar::Program::from_str($source)
				.context("Failed to build a program.")?;
			assert_eq!(program.run(), $result);
		};
	}

	#[test]
	fn test_vm() -> Result<()> {
		use {crate::compile::vm::Instruct, std::str::FromStr as _};

		assert_run!("1.3 + 2.1", 3.4000000000000004);
		assert_run!("2.5 / 2", 1.25);
		assert_run!("2.1 * (5.3 - 2.2 / 4)", 9.975);
		Ok(())
	}
}
