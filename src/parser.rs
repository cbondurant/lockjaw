use std::collections::VecDeque;

use crate::lexer::{Lexeme, LexemeType};

#[derive(Debug, Clone, Copy)]
pub enum LockjawParseError {
	InvalidOperator { index: usize },
	InvalidLiteral { index: usize },
}

#[derive(Debug, Clone, Copy)]
pub enum LockjawRuntimeError<'a> {
	InvalidArguments(&'a str),
	InvalidArgumentCount(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub enum Symbol {
	Plus,
	Minus,
	Multiply,
	Divide,
	Quote,
	Car,
	Cdr,
	Join,
	Eval,
}

impl Symbol {
	pub fn parse(lexeme: Lexeme) -> Result<Self, LockjawParseError> {
		match lexeme.value {
			LexemeType::Plus => Ok(Self::Plus),
			LexemeType::Dash => Ok(Self::Minus),
			LexemeType::Asterisk => Ok(Self::Multiply),
			LexemeType::ForwardSlash => Ok(Self::Divide),
			_ => Err(LockjawParseError::InvalidOperator {
				index: lexeme.index,
			}),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum AtomType<'a> {
	Float(f64),
	Int(i64),
	Symbol(Symbol),
	Err(LockjawRuntimeError<'a>),
}

#[derive(Debug)]
pub enum Expression<'a> {
	Atom(AtomType<'a>),
	SExpression(VecDeque<Expression<'a>>),
	QExpression(VecDeque<Expression<'a>>),
}

impl<'a> Expression<'a> {
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
		if LexemeType::LeftParen == lexemes[0].value {
			let mut exprlist = VecDeque::new();
			let mut current_lexeme = 1;
			while current_lexeme < lexemes.len()
				&& LexemeType::RightParen != lexemes[current_lexeme].value()
			{
				let expression = Expression::parse(&lexemes[current_lexeme..])?;
				current_lexeme += expression.lexeme_len();
				exprlist.push_back(expression);
			}
			Ok(Self::SExpression(exprlist))
		} else if LexemeType::LeftCBracket == lexemes[0].value {
			let mut exprlist = VecDeque::new();
			let mut current_lexeme = 1;
			while current_lexeme < lexemes.len()
				&& LexemeType::RightCBracket != lexemes[current_lexeme].value()
			{
				let expression = Expression::parse(&lexemes[current_lexeme..])?;
				current_lexeme += expression.lexeme_len();
				exprlist.push_back(expression);
			}
			Ok(Self::QExpression(exprlist))
		} else {
			Ok(Expression::Atom(match lexemes[0].value {
				LexemeType::Integer(value) => AtomType::Int(value),
				LexemeType::Float(value) => AtomType::Float(value),
				non_lit => AtomType::Symbol(match non_lit {
					LexemeType::Plus => Symbol::Plus,
					LexemeType::Dash => Symbol::Minus,
					LexemeType::Asterisk => Symbol::Multiply,
					LexemeType::ForwardSlash => Symbol::Divide,
					LexemeType::RawSymbol(str) => match str.to_ascii_lowercase().as_str() {
						"quote" => Symbol::Quote,
						"car" => Symbol::Car,
						"cdr" => Symbol::Cdr,
						"join" => Symbol::Join,
						"eval" => Symbol::Eval,
						_ => {
							return Err(LockjawParseError::InvalidOperator {
								index: lexemes[0].index,
							})
						}
					},
					_ => {
						return Err(LockjawParseError::InvalidLiteral {
							index: lexemes[0].index,
						})
					}
				}),
			}))
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
