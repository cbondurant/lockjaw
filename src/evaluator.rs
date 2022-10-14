use crate::builtins;
use crate::environment::Environment;
use crate::types::*;
use std::collections::VecDeque;

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
			Ok(Expression::SExpression(VecDeque::new()))
		}
	}

	pub fn new() -> Self {
		let mut env: Environment = Environment::new();
		env.def("+".to_string(), Value::Builtin(builtins::add));
		env.def("-".to_string(), Value::Builtin(builtins::sub));
		env.def("*".to_string(), Value::Builtin(builtins::mul));
		env.def("/".to_string(), Value::Builtin(builtins::div));
		env.def("car".to_string(), Value::Builtin(builtins::car));
		env.def("cdr".to_string(), Value::Builtin(builtins::cdr));
		env.def("join".to_string(), Value::Builtin(builtins::join));
		env.def("eval".to_string(), Value::Eval);
		env.def("def".to_string(), Value::Def);
		env.def("fun".to_string(), Value::Builtin(builtins::fun));

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
				let mut curriedfunc = func.clone();
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
			return Ok(Expression::SExpression(expressions));
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
