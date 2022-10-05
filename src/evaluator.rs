use crate::parser::{Atom, Expression};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockjawRuntimeError<'a> {
	InvalidArguments(&'a str),
	InvalidArgumentCount(&'a str),
	UnboundExpression,
}

type Function = fn(VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError>;

pub struct Evaluator {
	env: HashMap<String, Function>,
}

impl<'a> Evaluator {
	fn add(args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.is_empty() {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"+ requires at least one argument.",
			));
		}
		let mut accumulator = Atom::Int(0);
		for expr in args {
			accumulator = match expr {
				Expression::SExpression(_) => Err(LockjawRuntimeError::InvalidArguments(
					"SExpr cannot be an argument for +",
				)),
				Expression::QExpression(_) => Err(LockjawRuntimeError::InvalidArguments(
					"QExpr cannot be an argument for +",
				)),
				Expression::Atom(a) => match (a, accumulator) {
					(Atom::Float(f), Atom::Float(g)) => Ok(Atom::Float(f + g)),
					(Atom::Float(f), Atom::Int(i)) => Ok(Atom::Float(i as f64 + f)),
					(Atom::Int(i), Atom::Float(f)) => Ok(Atom::Float(i as f64 + f)),
					(Atom::Int(i), Atom::Int(j)) => Ok(Atom::Int(i + j)),
					(_, Atom::Symbol(_)) => unreachable!(),
					(Atom::Symbol(_), _) => Err(LockjawRuntimeError::InvalidArguments(
						"Cannot add a non-number",
					)),
				},
			}?
		}
		Ok(Expression::Atom(accumulator))
	}

	fn sub(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.is_empty() {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"- requires at least one argument.",
			));
		}
		let mut accumulator = match args.pop_front().unwrap() {
			Expression::Atom(val) => match val {
				Atom::Float(f) => Atom::Float(f),
				Atom::Int(i) => Atom::Int(i),
				Atom::Symbol(_) => {
					return Err(LockjawRuntimeError::InvalidArguments(
						"Expected number got Symbol",
					))
				}
			},
			Expression::SExpression(_) => {
				return Err(LockjawRuntimeError::InvalidArguments(
					"Expected number got SExpr",
				))
			}
			Expression::QExpression(_) => {
				return Err(LockjawRuntimeError::InvalidArguments(
					"Expected number got SExpr",
				))
			}
		};

		if args.is_empty() {
			return Ok(Expression::Atom(match accumulator {
				Atom::Float(f) => Atom::Float(-f),
				Atom::Int(i) => Atom::Int(-i),
				Atom::Symbol(_) => unreachable!(),
			}));
		}

		for expr in args {
			accumulator = match expr {
				Expression::SExpression(_) => Err(LockjawRuntimeError::InvalidArguments(
					"Expected number got SExpr",
				)),
				Expression::QExpression(_) => Err(LockjawRuntimeError::InvalidArguments(
					"Expected number got SExpr",
				)),
				Expression::Atom(a) => match (accumulator, a) {
					(Atom::Float(f), Atom::Float(g)) => Ok(Atom::Float(f - g)),
					(Atom::Float(f), Atom::Int(i)) => Ok(Atom::Float(i as f64 - f)),
					(Atom::Int(i), Atom::Float(f)) => Ok(Atom::Float(i as f64 - f)),
					(Atom::Int(i), Atom::Int(j)) => Ok(Atom::Int(i - j)),
					(_, Atom::Symbol(_)) => unreachable!(),
					(Atom::Symbol(_), _) => Err(LockjawRuntimeError::InvalidArguments(
						"expected number got Symbol",
					)),
				},
			}?
		}
		Ok(Expression::Atom(accumulator))
	}

	fn mul(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.is_empty() {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"+ requires at least one argument.",
			));
		}
		let mut accumulator = match args.pop_front().unwrap() {
			Expression::Atom(val) => match val {
				Atom::Float(f) => Atom::Float(f),
				Atom::Int(i) => Atom::Int(i),
				Atom::Symbol(_) => {
					return Err(LockjawRuntimeError::InvalidArguments(
						"Expected number got Symbol",
					))
				}
			},
			Expression::SExpression(_) => {
				return Err(LockjawRuntimeError::InvalidArguments(
					"Expected number got SExpr",
				))
			}
			Expression::QExpression(_) => {
				return Err(LockjawRuntimeError::InvalidArguments(
					"Expected number got SExpr",
				))
			}
		};
		for expr in args {
			accumulator = match expr {
				Expression::SExpression(_) => Err(LockjawRuntimeError::InvalidArguments(
					"SExpr cannot be an argument for *",
				)),
				Expression::QExpression(_) => Err(LockjawRuntimeError::InvalidArguments(
					"QExpr cannot be an argument for *",
				)),
				Expression::Atom(a) => match (a, accumulator) {
					(Atom::Float(f), Atom::Float(g)) => Ok(Atom::Float(f * g)),
					(Atom::Float(f), Atom::Int(i)) => Ok(Atom::Float(i as f64 * f)),
					(Atom::Int(i), Atom::Float(f)) => Ok(Atom::Float(i as f64 * f)),
					(Atom::Int(i), Atom::Int(j)) => Ok(Atom::Int(i * j)),
					(_, Atom::Symbol(_)) => unreachable!(),
					(Atom::Symbol(_), _) => Err(LockjawRuntimeError::InvalidArguments(
						"Cannot multiply a non-number",
					)),
				},
			}?
		}
		Ok(Expression::Atom(accumulator))
	}

	fn div(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.is_empty() {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"+ requires at least one argument.",
			));
		}
		let mut accumulator = match args.pop_front().unwrap() {
			Expression::Atom(val) => match val {
				Atom::Float(f) => Atom::Float(f),
				Atom::Int(i) => Atom::Int(i),
				Atom::Symbol(_) => {
					return Err(LockjawRuntimeError::InvalidArguments(
						"Expected number got Symbol",
					))
				}
			},
			Expression::SExpression(_) => {
				return Err(LockjawRuntimeError::InvalidArguments(
					"Expected number got SExpr",
				))
			}
			Expression::QExpression(_) => {
				return Err(LockjawRuntimeError::InvalidArguments(
					"Expected number got SExpr",
				))
			}
		};
		for expr in args {
			accumulator = match expr {
				Expression::SExpression(_) => Err(LockjawRuntimeError::InvalidArguments(
					"SExpr cannot be an argument for /",
				)),
				Expression::QExpression(_) => Err(LockjawRuntimeError::InvalidArguments(
					"QExpr cannot be an argument for /",
				)),
				Expression::Atom(a) => match (accumulator, a) {
					(Atom::Float(f), Atom::Float(g)) => Ok(Atom::Float(f / g)),
					(Atom::Float(f), Atom::Int(i)) => Ok(Atom::Float(f / i as f64)),
					(Atom::Int(i), Atom::Float(f)) => Ok(Atom::Float(i as f64 / f)),
					(Atom::Int(i), Atom::Int(j)) => Ok(Atom::Float(i as f64 / j as f64)),
					(_, Atom::Symbol(_)) => unreachable!(),
					(Atom::Symbol(_), _) => Err(LockjawRuntimeError::InvalidArguments(
						"Cannot divide a non-number",
					)),
				},
			}?
		}
		Ok(Expression::Atom(accumulator))
	}

	fn quote(args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		return Ok(Expression::QExpression(args));
	}

	fn car(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.len() != 1 {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"Car only takes one argument.",
			));
		}
		if let Expression::QExpression(mut args) = args.pop_front().unwrap() {
			if args.is_empty() {
				Ok(Expression::QExpression(args))
			} else {
				Ok(args.pop_front().unwrap())
			}
		} else {
			Err(LockjawRuntimeError::InvalidArguments(
				"Car can only operate on qexpr",
			))
		}
	}

	fn cdr(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.len() != 1 {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"Cdr only takes one argument.",
			));
		}
		if let Expression::QExpression(mut args) = args.pop_front().unwrap() {
			if args.is_empty() {
				Ok(Expression::QExpression(args))
			} else {
				args.pop_front().unwrap();
				Ok(Expression::QExpression(args))
			}
		} else {
			Err(LockjawRuntimeError::InvalidArguments(
				"Cdr can only operate on qexpr",
			))
		}
	}

	fn join(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
		if args.len() != 2 {
			return Err(LockjawRuntimeError::InvalidArgumentCount(
				"Join requires two arguments.",
			));
		}

		match (args.pop_front().unwrap(), args.pop_front().unwrap()) {
			(Expression::QExpression(mut a), Expression::QExpression(mut b)) => {
				a.append(&mut b);
				Ok(Expression::QExpression(a))
			}
			_ => Err(LockjawRuntimeError::InvalidArguments(
				"Join can only operate on qexpr.",
			)),
		}
	}

	pub fn new() -> Self {
		let mut env: HashMap<String, Function> = HashMap::new();
		env.insert("+".to_string(), Self::add);
		env.insert("-".to_string(), Self::sub);
		env.insert("*".to_string(), Self::mul);
		env.insert("/".to_string(), Self::div);
		env.insert("quote".to_string(), Self::quote);
		env.insert("car".to_string(), Self::car);
		env.insert("cdr".to_string(), Self::cdr);
		env.insert("join".to_string(), Self::join);

		Evaluator { env }
	}

	fn resolve_sexpression(
		&'a self,
		mut expressions: VecDeque<Expression<'a>>,
	) -> Result<Expression, LockjawRuntimeError> {
		if expressions.is_empty() {
			return Ok(Expression::SExpression(expressions));
		}

		// Must check for Quote before any evaluation is done

		if let Expression::Atom(Atom::Symbol("quote")) = expressions[0] {
			expressions.pop_front();
			return Ok(Expression::QExpression(expressions));
		}

		let mut evals = VecDeque::new();

		for expression in expressions {
			evals.push_back(self.evaluate(expression)?);
		}

		if let Expression::Atom(Atom::Symbol(s)) = evals[0] {
			if let Some(e) = self.env.get(s) {
				evals.pop_front(); // Remove operator from list
				e(evals)
			} else {
				if let Expression::Atom(Atom::Symbol("eval")) = evals[0] {
					evals.pop_front();
					return if let Some(Expression::QExpression(exprlist)) = evals.pop_front() {
						self.resolve_sexpression(exprlist)
					} else {
						Err(LockjawRuntimeError::InvalidArguments(
							"Can only eval qexprs!",
						))
					};
				}

				return Err(LockjawRuntimeError::UnboundExpression);
			}
		} else {
			Err(LockjawRuntimeError::InvalidArguments("Invalid Operation"))
		}
	}

	pub fn evaluate(
		&'a self,
		expression: Expression<'a>,
	) -> Result<Expression, LockjawRuntimeError> {
		match expression {
			Expression::Atom(a) => Ok(Expression::Atom(a)),
			Expression::QExpression(q) => Ok(Expression::QExpression(q)),
			Expression::SExpression(expressions) => self.resolve_sexpression(expressions),
		}
	}
}
