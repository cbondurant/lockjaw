use crate::{numeric::Numeric, types::*};
use std::collections::{HashMap, VecDeque};

pub struct Evaluator {
	env: HashMap<String, Value>,
}

impl<'a> Evaluator {
	fn add(args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.is_empty() {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"+ requires at least one argument.".to_string(),
			));
		}
		let mut accumulator = Atom::Number(Numeric::Int(0));
		for expr in args {
			let value = expr.get_atom()?;
			accumulator = match (value, accumulator) {
				(Atom::Number(a), Atom::Number(b)) => Atom::Number(a + b),
				_ => {
					return Err(LockjawRuntimeError::InvalidArguments(
						"Cannot add non-number".to_string(),
					))
				}
			};
		}

		Ok(Expression::Atom(accumulator))
	}

	fn sub(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.is_empty() {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"- requires at least one argument.".to_string(),
			));
		}
		let mut accumulator = args.pop_front().unwrap().get_atom()?;

		if args.is_empty() {
			return Ok(Expression::Atom(match accumulator {
				Atom::Number(num) => Atom::Number(-num),
				_ => {
					return Err(LockjawRuntimeError::InvalidArguments(
						"Cannot negate non-number".to_string(),
					))
				}
			}));
		}

		for expr in args {
			let value = expr.get_atom()?;
			accumulator = match (accumulator, value) {
				(Atom::Number(a), Atom::Number(b)) => Atom::Number(a - b),
				_ => {
					return Err(LockjawRuntimeError::InvalidArguments(
						"Cannot Subtract non-number".to_string(),
					))
				}
			};
		}
		Ok(Expression::Atom(accumulator))
	}

	fn mul(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.is_empty() {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"+ requires at least one argument.".to_string(),
			));
		}
		let mut accumulator = args.pop_front().unwrap().get_atom()?;
		for expr in args {
			let value = expr.get_atom()?;
			accumulator = match (value, accumulator) {
				(Atom::Number(a), Atom::Number(b)) => Atom::Number(a * b),
				_ => {
					return Err(LockjawRuntimeError::InvalidArguments(
						"Cannot multiply a non-number".to_string(),
					))
				}
			};
		}
		Ok(Expression::Atom(accumulator))
	}

	fn div(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.is_empty() {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"/ requires at least one argument.".to_string(),
			));
		}

		let mut accumulator = args.pop_front().unwrap().get_atom()?;

		for expr in args {
			let value = expr.get_atom()?;
			accumulator = match (accumulator, value) {
				(Atom::Number(a), Atom::Number(b)) => Atom::Number(a / b),
				_ => {
					return Err(LockjawRuntimeError::InvalidArguments(
						"Cannot divide a non-number".to_string(),
					))
				}
			};
		}
		Ok(Expression::Atom(accumulator))
	}

	fn car(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.len() != 1 {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"Car only takes one argument.".to_string(),
			));
		}

		let mut args = args.pop_front().unwrap().get_from_q_expression()?;
		if args.is_empty() {
			Ok(Expression::QExpression(args))
		} else {
			Ok(args.pop_front().unwrap())
		}
	}

	fn cdr(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.len() != 1 {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"Cdr only takes one argument.".to_string(),
			));
		}

		let mut args = args.pop_front().unwrap().get_from_q_expression()?;

		if !args.is_empty() {
			args.pop_front().unwrap();
		}

		Ok(Expression::QExpression(args))
	}

	fn join(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.len() != 2 {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"Join requires two arguments.".to_string(),
			));
		}

		let mut a = args.pop_front().unwrap().get_from_q_expression()?;
		let mut b = args.pop_front().unwrap().get_from_q_expression()?;

		a.append(&mut b);
		Ok(Expression::QExpression(a))
	}

	fn def(
		&'a mut self,
		mut args: VecDeque<Expression>,
	) -> Result<Expression, LockjawRuntimeError> {
		let expressions = args.pop_front().unwrap().get_from_q_expression()?;
		if expressions.len() != args.len() {
			Err(LockjawRuntimeError::InvalidArgumentCount(
				"Def requires one value per variable name".to_string(),
			))
		} else {
			for (token, value) in expressions.iter().zip(args.iter()) {
				if let Expression::Atom(Atom::Symbol(phrase)) = token {
					self.env
						.insert(phrase.to_string(), Value::Variable(Box::new(value.clone())));
					/*
					.get_mut(k)
					.insert(phrase.to_string(), Function::Defined(value.clone()));*/
				}
			}
			Ok(Expression::SExpression(VecDeque::new()))
		}
	}

	pub fn new() -> Self {
		let mut env: HashMap<String, Value> = HashMap::new();
		env.insert("+".to_string(), Value::Builtin(Self::add));
		env.insert("-".to_string(), Value::Builtin(Self::sub));
		env.insert("*".to_string(), Value::Builtin(Self::mul));
		env.insert("/".to_string(), Value::Builtin(Self::div));
		env.insert("car".to_string(), Value::Builtin(Self::car));
		env.insert("cdr".to_string(), Value::Builtin(Self::cdr));
		env.insert("join".to_string(), Value::Builtin(Self::join));
		env.insert("eval".to_string(), Value::Eval);
		env.insert("def".to_string(), Value::Def);

		Evaluator { env }
	}

	fn resolve_sexpression(
		&'a mut self,
		mut expressions: VecDeque<Expression>,
	) -> Result<Expression, LockjawRuntimeError> {
		if expressions.is_empty() {
			return Ok(Expression::SExpression(expressions));
		}

		// Must check for Quote before any evaluation is done
		if let Some(Expression::Atom(Atom::Symbol(sym))) = expressions.get(0) {
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
		}
	}

	fn evaluate_symbol(&self, symb: &str) -> Result<Expression, LockjawRuntimeError> {
		match self.env.get(symb) {
			Some(Value::Variable(e)) => Ok(*e.clone()), // Prevent nesting atoms in values in atoms
			Some(val) => Ok(Expression::Atom(Atom::Value(val.clone()))),
			None => Err(LockjawRuntimeError::UnboundExpression),
		}
	}

	pub fn evaluate(
		&'a mut self,
		expression: Expression,
	) -> Result<Expression, LockjawRuntimeError> {
		match &expression {
			Expression::Atom(Atom::Symbol(symb)) => self.evaluate_symbol(symb),
			Expression::Atom(_) => Ok(expression),
			Expression::SExpression(expressions) => self.resolve_sexpression(expressions.clone()),
			_ => Ok(expression),
		}
	}
}
