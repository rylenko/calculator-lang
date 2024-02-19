lazy_static::lazy_static! {
	pub static ref PRATT: pest::pratt_parser::PrattParser<Rule> = {
		use pest::pratt_parser::{Assoc::Left, Op};

		pest::pratt_parser::PrattParser::new()
			.op(Op::infix(Rule::Add, Left) | Op::infix(Rule::Substract, Left))
			.op(Op::infix(Rule::Multiply, Left) | Op::infix(Rule::Divide, Left))
	};
}

pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;
