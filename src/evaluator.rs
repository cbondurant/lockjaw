use crate::builtins;
use crate::environment::Environment;
use crate::parser;
use crate::types::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Evaluator {
	env: Environment,
}

impl Evaluator {
	fn def(&mut self, mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		let expressions = args.pop_front().unwrap().get_from_q_expression()?;
		if expressions.len() != args.len() {
			Err(LockjawRuntimeError::InvalidArgumentCount(
				"Def requires one value per variable name".to_string(),
			))
		} else {
			for (token, value) in expressions.iter().zip(args.iter()) {
				if let Expression::Atom(Atom::Symbol(phrase)) = token {
					self.env
						.def(phrase.to_string(), Value::Variable(Box::new(value.clone())));
				}
			}
			Ok(Expression::Null)
		}
	}

	pub fn cond(&mut self, args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.is_empty() {
			return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
				"At least one condition is required for a cond statement.",
			)));
		}

		for arg in args {
			let mut qexpr = arg.get_from_q_expression()?;
			if qexpr.len() != 2 {
				return Err(LockjawRuntimeError::InvalidArguments(String::from(
					"All Conditions must have one query and one predicate.",
				)));
			}
			let query_result = self.evaluate(qexpr.pop_front().unwrap())?;
			match query_result {
				Expression::Atom(Atom::Bool(true)) => {
					let eval = self.evaluate(qexpr.pop_front().unwrap())?;
					return Ok(eval);
				}
				_ => continue,
			}
		}
		Err(LockjawRuntimeError::CondFailure)
	}

	pub fn load(
		&mut self,
		mut args: VecDeque<Expression>,
	) -> Result<Expression, LockjawRuntimeError> {
		if args.len() != 1 {
			return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
				"load takes a single file name as input.",
			)));
		}

		if let Expression::Atom(Atom::String(path)) = args.pop_front().unwrap() {
			let path = Path::new(path.as_str());
			let mut f = File::open(path)?;
			let mut s = String::new();

			f.read_to_string(&mut s)?;
			let expression = parser::Parser::parse_from_text(s.as_str())?;
			println!("{:#?}", expression);
			match expression {
				Expression::SExpression(statements) => {
					for e in statements {
						self.evaluate(e)?;
					}
					Ok(Expression::Null)
				}
				Expression::Atom(_) => Ok(expression),
				Expression::QExpression(_) => Ok(expression),
				Expression::Null => Ok(expression),
			}
		} else {
			Ok(Expression::Null)
		}
	}

	pub fn new() -> Self {
		let mut env: Environment = Environment::new();
		env.def(String::from("+"), Value::Builtin(builtins::add));
		env.def(String::from("-"), Value::Builtin(builtins::sub));
		env.def(String::from("*"), Value::Builtin(builtins::mul));
		env.def(String::from("/"), Value::Builtin(builtins::div));
		env.def(String::from("car"), Value::Builtin(builtins::car));
		env.def(String::from("cdr"), Value::Builtin(builtins::cdr));
		env.def(String::from("join"), Value::Builtin(builtins::join));

		// These are special functions that depend on mutating self, and need to be treated special as such.
		env.def(String::from("eval"), Value::Eval);
		env.def(String::from("def"), Value::Def);
		env.def(String::from("cond"), Value::Cond);
		env.def(String::from("load"), Value::Load);
		env.def(String::from("fun"), Value::Builtin(builtins::fun));
		env.def(String::from("null?"), Value::Builtin(builtins::null_q));
		env.def(String::from("atom?"), Value::Builtin(builtins::atom_q));
		env.def(String::from("and?"), Value::Builtin(builtins::and_q));
		env.def(String::from("or?"), Value::Builtin(builtins::or_q));
		env.def(String::from("xor?"), Value::Builtin(builtins::xor_q));
		env.def(String::from("gt?"), Value::Builtin(builtins::gt_q));
		env.def(String::from("lt?"), Value::Builtin(builtins::lt_q));
		env.def(String::from("eq?"), Value::Builtin(builtins::eq_q));
		env.def(String::from("zero?"), Value::Builtin(builtins::zero_q));
		env.def(
			String::from("#f"),
			Value::Variable(Box::new(Expression::Atom(Atom::Bool(false)))),
		);
		env.def(
			String::from("#t"),
			Value::Variable(Box::new(Expression::Atom(Atom::Bool(true)))),
		);
		env.def(
			String::from("else"),
			Value::Variable(Box::new(Expression::Atom(Atom::Bool(true)))),
		);

		Evaluator { env }
	}

	fn evaluate_user_func(
		&mut self,
		func: UserFunc,
		args: VecDeque<Expression>,
	) -> Result<Expression, LockjawRuntimeError> {
		// Evaluate if we have enough arguments.
		match func.args.len().cmp(&(func.curried.len() + args.len())) {
			std::cmp::Ordering::Equal => {
				// Move into child environment
				self.env.push_env();
				let args = func.curried.iter().chain(args.iter());
				for (arg, argv) in func.args.iter().zip(args) {
					if let Expression::Atom(Atom::Symbol(symb)) = arg {
						self.env
							.put(symb.clone(), Value::Variable(Box::new(argv.clone())));
					}
				}

				// Evaluate Function
				let value = self.resolve_sexpression(func.body)?;
				// Move out of child environment
				self.env.pop_env();
				Ok(value)
			}
			std::cmp::Ordering::Greater => {
				let mut curriedfunc = func;
				curriedfunc.curried.extend(args);
				Ok(Expression::Atom(Atom::Value(Value::UserDef(curriedfunc))))
			}
			std::cmp::Ordering::Less => Err(LockjawRuntimeError::InvalidArgumentCount(
				String::from("Too many arguments for function!"),
			)),
		}
	}

	fn resolve_sexpression(
		&mut self,
		mut expressions: VecDeque<Expression>,
	) -> Result<Expression, LockjawRuntimeError> {
		if expressions.is_empty() {
			return Ok(Expression::Null);
		}

		// Must check for Quote before any evaluation is done
		if let Some(Expression::Atom(Atom::Symbol(ref sym))) = expressions.get(0) {
			if sym == "quote" {
				expressions.pop_front();
				return Ok(Expression::QExpression(expressions));
			}
		}

		let mut evals = VecDeque::new();

		for expression in expressions {
			evals.push_back(self.evaluate(expression)?);
		}

		// If single symbol, attempt to resolve.
		if evals.len() == 1 {
			return Ok(evals.pop_front().unwrap());
		}

		let val = evals.pop_front().unwrap().get_atom()?.get_as_value()?;
		match val {
			Value::Builtin(f) => f(evals),
			Value::Eval => {
				self.resolve_sexpression(evals.pop_front().unwrap().get_from_q_expression()?)
			}
			Value::Def => self.def(evals),
			Value::Cond => self.cond(evals),
			Value::Load => self.load(evals),
			Value::Variable(_) => Err(LockjawRuntimeError::InvalidFunction(format!(
				"Expected Function, got {}",
				val
			))),
			Value::UserDef(func) => self.evaluate_user_func(func, evals),
		}
	}

	fn evaluate_symbol(&self, symb: &str) -> Result<Expression, LockjawRuntimeError> {
		match self.env.get(symb.to_string()) {
			Some(Value::Variable(e)) => Ok(*e.clone()), // Prevent nesting atoms in values in atoms
			Some(val) => Ok(Expression::Atom(Atom::Value(val.clone()))),
			None => Err(LockjawRuntimeError::UnboundExpression),
		}
	}

	pub fn evaluate(&mut self, expression: Expression) -> Result<Expression, LockjawRuntimeError> {
		match expression {
			Expression::Atom(Atom::Symbol(ref symb)) => self.evaluate_symbol(symb),
			Expression::Atom(_) => Ok(expression),
			Expression::SExpression(expressions) => self.resolve_sexpression(expressions),
			_ => Ok(expression),
		}
	}
}
