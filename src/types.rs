use crate::numeric::Numeric;
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockjawRuntimeError {
	InvalidArguments(String),
	InvalidArgumentCount(String),
	InvalidFunction(String),
	UnboundExpression,
}

type BuiltinFunction = fn(VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Builtin(BuiltinFunction),
	// Special values because they have special clling conventions for funcs
	Eval,
	Def,
	Variable(Box<Expression>),
}

impl Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Variable(e) => write!(f, "{}", *e),
			Value::Eval | Value::Def | Value::Builtin(_) => write!(f, "<BUILTIN_FUNC>"),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
	Number(Numeric),
	Symbol(String),
	Value(Value),
}

impl Atom {
	#[allow(dead_code)]
	pub fn get_as_symbol(self) -> Result<String, LockjawRuntimeError> {
		if let Atom::Symbol(symb) = self {
			Ok(symb)
		} else {
			Err(LockjawRuntimeError::InvalidArguments(format!(
				"Expected Symbol, got {}",
				self
			)))
		}
	}

	pub fn get_as_value(self) -> Result<Value, LockjawRuntimeError> {
		if let Atom::Value(v) = self {
			Ok(v)
		} else {
			Err(LockjawRuntimeError::InvalidArguments(format!(
				"Expected Value, got {}",
				self
			)))
		}
	}
}

impl Display for Atom {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Atom::Number(num) => write!(f, "Number: {}", num),
			Atom::Symbol(v) => write!(f, "Symbol: {}", v),
			Atom::Value(v) => write!(f, "{}", v),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	Atom(Atom),
	SExpression(VecDeque<Expression>),
	QExpression(VecDeque<Expression>),
}

impl Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expression::Atom(v) => write!(f, "{}", v),
			Expression::SExpression(v) => {
				write!(f, "( ")?;
				for expr in v {
					write!(f, "{} ", expr)?;
				}
				write!(f, ")")
			}
			Expression::QExpression(v) => {
				write!(f, "{{ ")?;
				for expr in v {
					write!(f, "{} ", expr)?;
				}
				write!(f, "}}")
			}
		}
	}
}

impl Expression {
	pub fn get_from_q_expression(self) -> Result<VecDeque<Expression>, LockjawRuntimeError> {
		match self {
			Expression::QExpression(val) => Ok(val),
			invalid => Err(LockjawRuntimeError::InvalidArguments(format!(
				"Expected QExpression, got {}",
				invalid
			))),
		}
	}
	pub fn get_atom(self) -> Result<Atom, LockjawRuntimeError> {
		match self {
			Expression::Atom(val) => Ok(val),
			invalid => Err(LockjawRuntimeError::InvalidArguments(format!(
				"Expected Atom, got {}",
				invalid
			))),
		}
	}
}
