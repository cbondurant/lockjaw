use crate::{
	numeric::Numeric,
	parser::{Atom, Expression},
};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockjawRuntimeError {
	InvalidArguments(String),
	InvalidArgumentCount(String),
	UnboundExpression,
}

type BuiltinFunction = fn(VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError>;

enum EnvVar {
	Builtin(BuiltinFunction),
	Variable(Expression),
}

pub struct Evaluator {
	env: HashMap<String, EnvVar>,
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
						.insert(phrase.to_string(), EnvVar::Variable(value.clone()));
					/*
					.get_mut(k)
					.insert(phrase.to_string(), Function::Defined(value.clone()));*/
				}
			}
			Ok(Expression::SExpression(VecDeque::new()))
		}
	}

	pub fn new() -> Self {
		let mut env: HashMap<String, EnvVar> = HashMap::new();
		env.insert("+".to_string(), EnvVar::Builtin(Self::add));
		env.insert("-".to_string(), EnvVar::Builtin(Self::sub));
		env.insert("*".to_string(), EnvVar::Builtin(Self::mul));
		env.insert("/".to_string(), EnvVar::Builtin(Self::div));
		env.insert("car".to_string(), EnvVar::Builtin(Self::car));
		env.insert("cdr".to_string(), EnvVar::Builtin(Self::cdr));
		env.insert("join".to_string(), EnvVar::Builtin(Self::join));

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

		// Just return single values
		if evals.len() == 1 {
			if let Some(e) = evals.pop_front() {
				return Ok(e);
			}
		}

		let symbol = evals.pop_front().unwrap().get_atom()?.get_as_symbol()?;

		match symbol.as_str() {
			"eval" => self.resolve_sexpression(evals.pop_front().unwrap().get_from_q_expression()?),
			"def" => self.def(evals),
			var => match self.env.get(var) {
				Some(EnvVar::Builtin(f)) => f(evals),
				Some(EnvVar::Variable(val)) => Ok(val.clone()),
				None => Err(LockjawRuntimeError::UnboundExpression),
			},
		}
	}

	pub fn evaluate(
		&'a mut self,
		expression: Expression,
	) -> Result<Expression, LockjawRuntimeError> {
		match &expression {
			Expression::Atom(Atom::Symbol(s)) => {
				if (s == "def") | (s == "eval") {
					Ok(expression)
				} else if let Some(t) = self.env.get(s) {
					match t {
						EnvVar::Builtin(_) => Ok(expression),
						EnvVar::Variable(val) => Ok(val.clone()),
					}
				} else {
					Err(LockjawRuntimeError::UnboundExpression)
				}
			}
			Expression::SExpression(expressions) => self.resolve_sexpression(expressions.clone()),
			_ => Ok(expression),
		}
	}
}
