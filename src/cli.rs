pub enum CompileMode {
	Interpret,
	Jit,
	Vm,
}

impl std::fmt::Display for CompileMode {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Interpret => write!(f, "interpret"),
			Self::Jit => write!(f, "jit"),
			Self::Vm => write!(f, "vm"),
		}
	}
}

impl From<Option<String>> for CompileMode {
	#[must_use]
	fn from(v: Option<String>) -> Self {
		match v.as_deref() {
			Some("interpret") => Self::Interpret,
			Some("jit") => Self::Jit,
			Some("vm") => Self::Vm,
			_ => Self::Interpret,
		}
	}
}

pub fn extract_compile_mode() -> CompileMode {
	CompileMode::from(std::env::args().nth(1))
}
