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
	pub fn evaluate(expression: Expression<'a>) -> Expression<'a> {
		match expression {
			Expression::Atom(a) => Expression::Atom(a),
			Expression::SExpression(expressions) => {
				let mut evals = VecDeque::new();
				for expression in expressions {
					evals.push_back(Self::evaluate(expression));
				}
				if evals.len() == 0 {
					return Expression::SExpression(VecDeque::new());
				}
				if evals.len() == 1 {
					return evals.pop_front().unwrap();
				}

				if evals.len() == 2 {
					if let Expression::Atom(AtomType::Symbol(Symbol::Minus)) = evals[0] {
						if let Expression::Atom(AtomType::Int(a)) = evals[1] {
							return Expression::Atom(AtomType::Int(-a));
						}
						if let Expression::Atom(AtomType::Float(a)) = evals[1] {
							return Expression::Atom(AtomType::Float(-a));
						}
					}
				}

				if let Some(Expression::Atom(AtomType::Symbol(s))) = evals.pop_front() {
					if let Some(Expression::Atom(mut accumulator)) = evals.pop_front() {
						for expr in evals {
							if let Expression::Atom(arg) = expr {
								match s {
									Symbol::Plus => accumulator = accumulator + arg,
									Symbol::Minus => accumulator = accumulator - arg,
									Symbol::Multiply => accumulator = accumulator * arg,
									Symbol::Divide => accumulator = accumulator / arg,
								}
							}
						}
						Expression::Atom(accumulator)
					} else {
						Expression::Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
							"Invalid argument",
						)))
					}
				} else {
					Expression::Atom(AtomType::Err(LockjawRuntimeError::InvalidArguments(
						"Invalid Operator",
					)))
				}
			}
		}
	}
}
