#[allow(dead_code)] // Used in `Jit::call`
type JitFunction = unsafe extern "C" fn() -> crate::grammar::Float;

pub trait Jit {
	#[must_use]
	fn jit<'a>(
		&self,
		float_type: inkwell::types::FloatType<'a>,
		builder: &'a inkwell::builder::Builder<'a>,
	) -> inkwell::values::FloatValue<'a>;

	fn call(&self) -> crate::grammar::Float {
		// Create context, module, builder and execution engine
		let context = inkwell::context::Context::create();
		let module = context.create_module("calculator");
		let builder = context.create_builder();
		let execution_engine = module
			.create_jit_execution_engine(inkwell::OptimizationLevel::None)
			.unwrap();

		// Create the function and it's block
		assert_eq!(std::any::type_name::<crate::grammar::Float>(), "f64");
		let f64_type = context.f64_type();
		let fn_type = f64_type.fn_type(&[], false);
		let fn_ = module.add_function("calculate", fn_type, None);
		let fn_basic_block =
			context.append_basic_block(fn_, "calculate_entry");

		// Build the function
		builder.position_at_end(fn_basic_block);
		let fn_return_value = self.jit(f64_type, &builder);
		builder.build_return(Some(&fn_return_value));

		unsafe {
			let jit_function = execution_engine
				.get_function::<JitFunction>("calculate")
				.unwrap();
			jit_function.call()
		}
	}
}

#[cfg(test)]
mod tests {
	use {
		super::Jit as _,
		anyhow::{Context as _, Result},
	};

	macro_rules! assert_call {
		($input:expr, $output:expr) => {
			assert_eq!(
				$output,
				Program::from_str($input)
					.with_context(|| format!(
						"Failed to build a program from {}.",
						$input
					))?
					.call()
			);
		};
	}

	#[test]
	fn test_jit() -> Result<()> {
		use {crate::grammar::Program, std::str::FromStr as _};

		assert_call!("-5/-1.0", 5.0);
		assert_call!("-(53/2) * 2", -53.0);
		assert_call!("2 - (2 + 2 * 2.3)", -4.6);
		assert_call!("2 + -(10 - 1)", -7.0);
		assert_call!("-(15 + (3 - (2 + 5 / 5) + 1))", -16.0);
		Ok(())
	}
}
