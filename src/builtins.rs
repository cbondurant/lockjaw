use crate::numeric::Numeric;
use crate::types::*;
use std::collections::VecDeque;

pub fn add(args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
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

pub fn sub(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
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

pub fn mul(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
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

pub fn div(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
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

pub fn car(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
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

pub fn cdr(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
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

pub fn join(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
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

pub fn fun(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 2 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"Functions require three arguments!",
		)));
	}

	// This is so messy and can crash out...
	let formals = args.pop_front().unwrap().get_from_q_expression()?;
	let body = args.pop_front().unwrap().get_from_q_expression()?;
	Ok(Expression::Atom(Atom::Value(Value::UserDef(UserFunc {
		args: formals,
		body,
		curried: VecDeque::new(),
	}))))
}

pub fn null_q(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 1 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"null? takes exactly one argument",
		)));
	}

	match args.pop_front() {
		Some(Expression::Null) => Ok(Expression::Atom(Atom::Bool(true))),
		Some(Expression::QExpression(v)) | Some(Expression::SExpression(v)) => {
			Ok(Expression::Atom(Atom::Bool(v.is_empty())))
		}
		Some(Expression::Atom(_)) => Ok(Expression::Atom(Atom::Bool(false))),
		None => unreachable!(),
	}
}

pub fn atom_q(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 1 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"null? takes exactly one argument",
		)));
	}

	match args.pop_front().unwrap() {
		Expression::Atom(_) => Ok(Expression::Atom(Atom::Bool(true))),
		_ => Ok(Expression::Atom(Atom::Bool(false))),
	}
}

pub fn and_q(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 2 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"and? takes exactly two arguments",
		)));
	}

	let a = args.pop_front().unwrap();
	let b = args.pop_front().unwrap();

	match (a, b) {
		(Expression::Atom(Atom::Bool(a)), Expression::Atom(Atom::Bool(b))) => {
			Ok(Expression::Atom(Atom::Bool(a & b)))
		}
		_ => Err(LockjawRuntimeError::InvalidArguments(String::from(
			"Arguments to and? must be bool.",
		))),
	}
}

pub fn or_q(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 2 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"or? takes exactly two arguments",
		)));
	}

	let a = args.pop_front().unwrap();
	let b = args.pop_front().unwrap();

	match (a, b) {
		(Expression::Atom(Atom::Bool(a)), Expression::Atom(Atom::Bool(b))) => {
			Ok(Expression::Atom(Atom::Bool(a | b)))
		}
		_ => Err(LockjawRuntimeError::InvalidArguments(String::from(
			"Arguments to or? must be bool.",
		))),
	}
}

pub fn xor_q(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 2 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"xor? takes exactly two arguments",
		)));
	}

	let a = args.pop_front().unwrap();
	let b = args.pop_front().unwrap();

	match (a, b) {
		(Expression::Atom(Atom::Bool(a)), Expression::Atom(Atom::Bool(b))) => {
			Ok(Expression::Atom(Atom::Bool(a ^ b)))
		}
		_ => Err(LockjawRuntimeError::InvalidArguments(String::from(
			"Arguments to xor? must be bool.",
		))),
	}
}

pub fn gt_q(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 2 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"gt? takes exactly two arguments",
		)));
	}

	let a = args.pop_front().unwrap().get_atom()?;
	let b = args.pop_front().unwrap().get_atom()?;

	match (a, b) {
		(Atom::Number(a), Atom::Number(b)) => Ok(Expression::Atom(Atom::Bool(a > b))),
		_ => Err(LockjawRuntimeError::InvalidArguments(String::from(
			"Arguments to gt? must be numeric.",
		))),
	}
}

pub fn lt_q(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 2 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"lt? takes exactly two arguments",
		)));
	}

	let a = args.pop_front().unwrap().get_atom()?;
	let b = args.pop_front().unwrap().get_atom()?;

	match (a, b) {
		(Atom::Number(a), Atom::Number(b)) => Ok(Expression::Atom(Atom::Bool(a < b))),
		_ => Err(LockjawRuntimeError::InvalidArguments(String::from(
			"Arguments to lt? must be numeric.",
		))),
	}
}

pub fn eq_q(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 2 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"eq? takes exactly two arguments",
		)));
	}

	let a = args.pop_front().unwrap().get_atom()?;
	let b = args.pop_front().unwrap().get_atom()?;

	match (a, b) {
		(Atom::Number(a), Atom::Number(b)) => Ok(Expression::Atom(Atom::Bool(a == b))),
		_ => Err(LockjawRuntimeError::InvalidArguments(String::from(
			"Arguments to eq? must be numeric.",
		))),
	}
}

pub fn zero_q(mut args: VecDeque<Expression>) -> Result<Expression, LockjawRuntimeError> {
	if args.len() != 1 {
		return Err(LockjawRuntimeError::InvalidArgumentCount(String::from(
			"xor? takes exactly one argument",
		)));
	}

	match args.pop_front().unwrap().get_atom()? {
		Atom::Number(a) => Ok(Expression::Atom(Atom::Bool(a == Numeric::Int(0)))),
		_ => Err(LockjawRuntimeError::InvalidArguments(String::from(
			"Arguments to gt? must be numeric.",
		))),
	}
}

// zero?
