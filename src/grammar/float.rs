pub type Float = f64;

impl crate::compile::interpret::Interpret for Float {
	#[inline]
	#[must_use]
	fn interpret(&self) -> Self {
		*self
	}
}

impl crate::compile::jit::Jit for Float {
	#[inline]
	#[must_use]
	fn jit<'a>(
		&self,
		float_type: inkwell::types::FloatType<'a>,
		_builder: &'a inkwell::builder::Builder<'a>,
	) -> inkwell::values::FloatValue<'a> {
		float_type.const_float(*self)
	}
}

impl crate::compile::vm::Instruct for Float {
	fn instruct(&self, bytecode: &mut crate::compile::vm::Bytecode) {
		bytecode.add_float(*self);
		let index = bytecode.floats().len() - 1;
		let instruction = crate::compile::vm::Instruction::FloatIndex(index);
		bytecode.add_instruction(instruction);
	}
}
