use crate::parser::{AtomType, Expression, LockjawRuntimeError, Symbol};
use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};

impl<'a> Add for AtomType<'a> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		match (self, other) {
			(Self::Int(a), Self::Int(b)) => Self::Int(a + b),
			(Self::Int(a), Self::Float(b)) => Self::Float(a as f64 + b),
			(Self::Float(a), Self::Int(b)) => Self::Float(a + b as f64),
			(Self::Float(a), Self::Float(b)) => Self::Float(a + b),
			(Self::Symbol(_), _) | (_, Self::Symbol(_)) => Self::Err(
				LockjawRuntimeError::InvalidArguments("Addition cannot apply to non-numbers."),
			),
			(Self::Err(e), _) => Self::Err(e),
			(_, Self::Err(e)) => Self::Err(e),
		}
	}
}

impl<'a> Sub for AtomType<'a> {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		match (self, other) {
			(Self::Int(a), Self::Int(b)) => Self::Int(a - b),
			(Self::Int(a), Self::Float(b)) => Self::Float(a as f64 - b),
			(Self::Float(a), Self::Int(b)) => Self::Float(a - b as f64),
			(Self::Float(a), Self::Float(b)) => Self::Float(a - b),
			(Self::Symbol(_), _) | (_, Self::Symbol(_)) => Self::Err(
				LockjawRuntimeError::InvalidArguments("Addition cannot apply to non-numbers."),
			),
			(Self::Err(e), _) => Self::Err(e),
			(_, Self::Err(e)) => Self::Err(e),
		}
	}
}

impl<'a> Mul for AtomType<'a> {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		match (self, other) {
			(Self::Int(a), Self::Int(b)) => Self::Int(a * b),
			(Self::Int(a), Self::Float(b)) => Self::Float(a as f64 * b),
			(Self::Float(a), Self::Int(b)) => Self::Float(a * b as f64),
			(Self::Float(a), Self::Float(b)) => Self::Float(a * b),
			(Self::Symbol(_), _) | (_, Self::Symbol(_)) => Self::Err(
				LockjawRuntimeError::InvalidArguments("Addition cannot apply to non-numbers."),
			),
			(Self::Err(e), _) => Self::Err(e),
			(_, Self::Err(e)) => Self::Err(e),
		}
	}
}

impl<'a> Div for AtomType<'a> {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		match (self, other) {
			(Self::Int(a), Self::Int(b)) => Self::Float(a as f64 / b as f64),
			(Self::Int(a), Self::Float(b)) => Self::Float(a as f64 / b),
			(Self::Float(a), Self::Int(b)) => Self::Float(a / b as f64),
			(Self::Float(a), Self::Float(b)) => Self::Float(a / b),
			(Self::Symbol(_), _) | (_, Self::Symbol(_)) => Self::Err(
				LockjawRuntimeError::InvalidArguments("Addition cannot apply to non-numbers."),
			),
			(Self::Err(e), _) => Self::Err(e),
			(_, Self::Err(e)) => Self::Err(e),
		}
	}
}

pub struct Evaluator {}

impl<'a> Evaluator {
	fn resolve_sexpression(mut expressions: VecDeque<Expression<'a>>) -> Expression<'a> {
		use Expression::*;

		// Must check for Quote before any evaluation is done

		if let Atom(AtomType::Symbol(Symbol::Quote)) = expressions[0] {
			expressions.pop_front();
			return QExpression(expressions);
		}

		if let Atom(AtomType::Symbol(Symbol::Eval)) = expressions[0] {
			expressions.pop_front();
			return if let Some(QExpression(exprlist)) = expressions.pop_front() {
				Self::resolve_sexpression(exprlist)
			} else {
				Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
					"Can only eval qexprs!",
				)))
			};
		}

		let mut evals = VecDeque::new();
		for expression in expressions {
			evals.push_back(Self::evaluate(expression));
		}
		if evals.is_empty() {
			return SExpression(VecDeque::new());
		}
		if evals.len() == 1 {
			return evals.pop_front().unwrap();
		}

		if evals.len() == 2 {
			match evals[0] {
				Atom(AtomType::Symbol(Symbol::Minus)) => match evals[1] {
					Atom(AtomType::Int(a)) => Atom(AtomType::Int(-a)),
					Atom(AtomType::Float(a)) => Atom(AtomType::Float(-a)),
					SExpression(_) => Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
						"Cannot negate non-number!",
					))),
					QExpression(_) => Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
						"Cannot negate non-number!",
					))),
					Atom(a) => Atom(a),
				},
				Atom(AtomType::Symbol(Symbol::Car)) => {
					evals.pop_front();
					if let Some(QExpression(mut elements)) = evals.pop_front() {
						let mut car = VecDeque::new();
						if let Some(val) = elements.pop_front() {
							car.push_back(val);
						}
						QExpression(car)
					} else {
						Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
							"Car requires a list as its argument!",
						)))
					}
				}
				Atom(AtomType::Symbol(Symbol::Cdr)) => {
					evals.pop_front();
					if let Some(QExpression(mut elements)) = evals.pop_front() {
						if elements.pop_front().is_some() {
							QExpression(elements)
						} else {
							Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
								"Unable to get cdr of empty expr",
							)))
						}
					} else {
						Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
							"Cdr requires a list as its argument!",
						)))
					}
				}
				SExpression(_) => todo!(),
				QExpression(_) => todo!(),
				Atom(a) => Atom(a),
			};
		}

		if let Atom(AtomType::Symbol(Symbol::Join)) = evals[0] {
			if evals.len() != 3 {
				return Atom(AtomType::Err(LockjawRuntimeError::InvalidArgumentCount(
					"Join takes exactly 2 arguments",
				)));
			}
			evals.pop_front(); // Remove join, we already have it.
			return if let Some(QExpression(mut a)) = evals.pop_front() {
				if let Some(QExpression(mut b)) = evals.pop_front() {
					a.append(&mut b);
					QExpression(a)
				} else {
					Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
						"Join requires two Quote expressions",
					)))
				}
			} else {
				Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
					"Join requires two Quote expressions",
				)))
			};
		}

		if let Some(Atom(AtomType::Symbol(s))) = evals.pop_front() {
			if let Some(Atom(mut accumulator)) = evals.pop_front() {
				for expr in evals {
					if let Atom(arg) = expr {
						match s {
							Symbol::Plus => accumulator = accumulator + arg,
							Symbol::Minus => accumulator = accumulator - arg,
							Symbol::Multiply => accumulator = accumulator * arg,
							Symbol::Divide => accumulator = accumulator / arg,
							_ => {
								return Atom(AtomType::Err(
									LockjawRuntimeError::InvalidArgumentCount(
										"Undefined Eval Error.",
									),
								))
							}
						}
					}
				}
				Atom(accumulator)
			} else {
				Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
					"Invalid argument",
				)))
			}
		} else {
			Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
				"Invalid Operation",
			)))
		}
	}

	pub fn evaluate(expression: Expression<'a>) -> Expression<'a> {
		match expression {
			Expression::Atom(a) => Expression::Atom(a),
			Expression::QExpression(q) => Expression::QExpression(q),
			Expression::SExpression(expressions) => Self::resolve_sexpression(expressions),
		}
	}
}
