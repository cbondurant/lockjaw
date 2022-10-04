use crate::parser::{Atom, Expression};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockjawRuntimeError<'a> {
	InvalidArguments(&'a str),
	ExpectedNumber,
	InvalidArgumentCount(&'a str),
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

	pub fn new() -> Self {
		let mut env: HashMap<String, Function> = HashMap::new();
		env.insert("+".to_string(), Self::add);
		env.insert("-".to_string(), Self::sub);
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

		if evals.is_empty() {
			return Ok(Expression::SExpression(VecDeque::new()));
		}

		if evals.len() == 1 {
			return Ok(evals.pop_front().unwrap());
		}

		if evals.len() == 2 {
			match evals[0] {
				Expression::Atom(Atom::Symbol("minus")) => match evals[1] {
					Expression::Atom(Atom::Int(a)) => Ok(Expression::Atom(Atom::Int(-a))),
					Expression::Atom(Atom::Float(a)) => Ok(Expression::Atom(Atom::Float(-a))),
					_ => Err(LockjawRuntimeError::InvalidArguments(
						"Cannot negate non-number!",
					)),
				},

				Expression::Atom(Atom::Symbol("car")) => {
					println!("{:?}", evals);
					evals.pop_front();
					return if let Some(Expression::QExpression(mut elements)) = evals.pop_front() {
						let mut car = VecDeque::new();
						if let Some(val) = elements.pop_front() {
							car.push_back(val);
						}
						Ok(Expression::QExpression(car))
					} else {
						Err(LockjawRuntimeError::InvalidArguments(
							"Car requires a list as its argument!",
						))
					};
				}
				Expression::Atom(Atom::Symbol("cdr")) => {
					evals.pop_front();
					if let Some(Expression::QExpression(mut elements)) = evals.pop_front() {
						if elements.pop_front().is_some() {
							Ok(Expression::QExpression(elements))
						} else {
							Err(LockjawRuntimeError::InvalidArguments(
								"Unable to get cdr of empty expr",
							))
						}
					} else {
						Err(LockjawRuntimeError::InvalidArguments(
							"Cdr requires a list as its argument!",
						))
					}
				}
				Expression::SExpression(_) => todo!(),
				Expression::QExpression(_) => todo!(),
				Expression::Atom(a) => Ok(Expression::Atom(a)),
			}?;
		}

		if let Expression::Atom(Atom::Symbol("join")) = evals[0] {
			if evals.len() != 3 {
				return Err(LockjawRuntimeError::InvalidArgumentCount(
					"Join takes exactly 2 arguments",
				));
			}
			evals.pop_front(); // Remove join, we already have it.
			return if let Some(Expression::QExpression(mut a)) = evals.pop_front() {
				if let Some(Expression::QExpression(mut b)) = evals.pop_front() {
					a.append(&mut b);
					Ok(Expression::QExpression(a))
				} else {
					Err(LockjawRuntimeError::InvalidArguments(
						"Join requires two Quote expressions",
					))
				}
			} else {
				Err(LockjawRuntimeError::InvalidArguments(
					"Join requires two Quote expressions",
				))
			};
		}

		if let Some(Expression::Atom(Atom::Symbol(s))) = evals.pop_front() {
			if let Some(e) = self.env.get(s) {
				e(evals)
			} else {
				unimplemented!()
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
