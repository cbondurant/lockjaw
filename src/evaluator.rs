use crate::parser::{Expression, NumberType, Operator};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum DataType {
	Int(i64),
	Float(f64),
}

impl Add for DataType {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		match (self, other) {
			(Self::Int(a), Self::Int(b)) => Self::Int(a + b),
			(Self::Int(a), Self::Float(b)) => Self::Float(a as f64 + b),
			(Self::Float(a), Self::Int(b)) => Self::Float(a + b as f64),
			(Self::Float(a), Self::Float(b)) => Self::Float(a + b),
		}
	}
}

impl Sub for DataType {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		match (self, other) {
			(Self::Int(a), Self::Int(b)) => Self::Int(a - b),
			(Self::Int(a), Self::Float(b)) => Self::Float(a as f64 - b),
			(Self::Float(a), Self::Int(b)) => Self::Float(a - b as f64),
			(Self::Float(a), Self::Float(b)) => Self::Float(a - b),
		}
	}
}

impl Mul for DataType {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		match (self, other) {
			(Self::Int(a), Self::Int(b)) => Self::Int(a * b),
			(Self::Int(a), Self::Float(b)) => Self::Float(a as f64 * b),
			(Self::Float(a), Self::Int(b)) => Self::Float(a * b as f64),
			(Self::Float(a), Self::Float(b)) => Self::Float(a * b),
		}
	}
}

impl Div for DataType {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		match (self, other) {
			(Self::Int(a), Self::Int(b)) => Self::Float(a as f64 / b as f64),
			(Self::Int(a), Self::Float(b)) => Self::Float(a as f64 / b),
			(Self::Float(a), Self::Int(b)) => Self::Float(a / b as f64),
			(Self::Float(a), Self::Float(b)) => Self::Float(a / b),
		}
	}
}

pub struct Evaluator {}

impl Evaluator {
	pub fn evaluate(expression: &Expression) -> DataType {
		match expression {
			Expression::Atom(NumberType::Int(n)) => DataType::Int(*n),
			Expression::Atom(NumberType::Float(f)) => DataType::Float(*f),
			Expression::SExpression { op, expressions } => expressions
				.iter()
				.map(Self::evaluate)
				.reduce(|x, y| match op {
					Operator::Plus => x + y,
					Operator::Minus => x - y,
					Operator::Multiply => x * y,
					Operator::Divide => x / y,
				})
				.unwrap_or_else(|| panic!("No arguments given!")),
		}
	}
}
