use std::{
	fmt::Display,
	ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Numeric {
	Float(f64),
	Int(i64),
}

impl Display for Numeric {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Numeric::Float(float) => write!(f, "{}", float),
			Numeric::Int(i) => write!(f, "{}", i),
		}
	}
}

impl Add for Numeric {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Numeric::Float(f1), Numeric::Float(f2)) => Numeric::Float(f1 + f2),
			(Numeric::Float(f), Numeric::Int(i)) => Numeric::Float(f + i as f64),
			(Numeric::Int(i), Numeric::Float(f)) => Numeric::Float(i as f64 + f),
			(Numeric::Int(i1), Numeric::Int(i2)) => Numeric::Int(i1 + i2),
		}
	}
}

impl Sub for Numeric {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Numeric::Float(f1), Numeric::Float(f2)) => Numeric::Float(f1 - f2),
			(Numeric::Float(f), Numeric::Int(i)) => Numeric::Float(f - i as f64),
			(Numeric::Int(i), Numeric::Float(f)) => Numeric::Float(i as f64 - f),
			(Numeric::Int(i1), Numeric::Int(i2)) => Numeric::Int(i1 - i2),
		}
	}
}

impl Mul for Numeric {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Numeric::Float(f1), Numeric::Float(f2)) => Numeric::Float(f1 * f2),
			(Numeric::Float(f), Numeric::Int(i)) => Numeric::Float(f * i as f64),
			(Numeric::Int(i), Numeric::Float(f)) => Numeric::Float(i as f64 * f),
			(Numeric::Int(i1), Numeric::Int(i2)) => Numeric::Int(i1 * i2),
		}
	}
}

impl Div for Numeric {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Numeric::Float(f1), Numeric::Float(f2)) => Numeric::Float(f1 / f2),
			(Numeric::Float(f), Numeric::Int(i)) => Numeric::Float(f / i as f64),
			(Numeric::Int(i), Numeric::Float(f)) => Numeric::Float(i as f64 / f),
			(Numeric::Int(i1), Numeric::Int(i2)) => Numeric::Float(i1 as f64 / i2 as f64),
		}
	}
}

impl Neg for Numeric {
	type Output = Self;

	fn neg(self) -> Self::Output {
		match self {
			Numeric::Float(f) => Numeric::Float(-f),
			Numeric::Int(i) => Numeric::Int(-i),
		}
	}
}

impl From<i64> for Numeric {
	fn from(i: i64) -> Self {
		Numeric::Int(i)
	}
}

impl From<f64> for Numeric {
	fn from(f: f64) -> Self {
		Numeric::Float(f)
	}
}
