use std::{collections::VecDeque, fmt::Display};

use crate::lexer::{Lexeme, LexemeType};

#[derive(Debug, Clone, Copy)]
pub enum LockjawParseError {
	InvalidLiteral { index: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
	Float(f64),
	Int(i64),
	Symbol(String),
}

impl<'a> Display for Atom {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Atom::Float(v) => write!(f, "Float: {}", v),
			Atom::Int(v) => write!(f, "Int: {}", v),
			Atom::Symbol(v) => write!(f, "Symbol: {}", v),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	Atom(Atom),
	SExpression(VecDeque<Expression>),
	QExpression(VecDeque<Expression>),
}

impl Expression {
	pub fn lexeme_len(&self) -> usize {
		match self {
			// Open paren, Operator, <expr list> Close Paren.
			Self::SExpression(exprlist) => {
				2 + exprlist.iter().map(Expression::lexeme_len).sum::<usize>()
			}
			Self::QExpression(exprlist) => {
				2 + exprlist.iter().map(Expression::lexeme_len).sum::<usize>()
			}
			Self::Atom(_) => 1,
		}
	}

	pub fn parse(lexemes: &[Lexeme]) -> Result<Self, LockjawParseError> {
		match lexemes[0].value {
			LexemeType::LeftParen => {
				let mut exprlist = VecDeque::new();
				let mut current_lexeme = 1;
				while current_lexeme < lexemes.len()
					&& LexemeType::RightParen != lexemes[current_lexeme].value
				{
					let expression = Expression::parse(&lexemes[current_lexeme..])?;
					current_lexeme += expression.lexeme_len();
					exprlist.push_back(expression);
				}
				Ok(Self::SExpression(exprlist))
			}
			LexemeType::LeftCBracket => {
				let mut exprlist = VecDeque::new();
				let mut current_lexeme = 1;
				while current_lexeme < lexemes.len()
					&& LexemeType::RightCBracket != lexemes[current_lexeme].value
				{
					let expression = Expression::parse(&lexemes[current_lexeme..])?;
					current_lexeme += expression.lexeme_len();
					exprlist.push_back(expression);
				}
				Ok(Self::QExpression(exprlist))
			}
			term => Ok(Expression::Atom(match term {
				LexemeType::Integer(value) => Atom::Int(value),
				LexemeType::Float(value) => Atom::Float(value),
				LexemeType::RawSymbol(symb) => Atom::Symbol(symb.to_string()),
				_ => {
					return Err(LockjawParseError::InvalidLiteral {
						index: lexemes[0].index,
					})
				}
			})),
		}
	}

	pub fn parse_root(lexemes: &[Lexeme]) -> Result<Self, LockjawParseError> {
		let mut expressions = VecDeque::new();
		let mut lexemes_consumed = 0;
		while lexemes_consumed < lexemes.len() {
			let expression = Expression::parse(&lexemes[lexemes_consumed..])?;
			lexemes_consumed += expression.lexeme_len();
			expressions.push_back(expression);
		}

		Ok(Self::SExpression(expressions))
	}
}
