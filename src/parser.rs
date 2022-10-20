use crate::lexer;
use std::collections::VecDeque;

use crate::{
	lexer::{Lexeme, LexemeType},
	numeric::Numeric,
	types::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockjawParseError {
	InvalidLiteral { index: usize },
	InvalidStringLiteral { code: char },
	UnexpectedEof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parser {
	ast: Expression,
}

impl Parser {
	pub fn lexeme_len(expr: &Expression) -> usize {
		match expr {
			// Open paren, Operator, <expr list> Close Paren.
			Expression::SExpression(exprlist) => {
				2 + exprlist.iter().map(Self::lexeme_len).sum::<usize>()
			}
			Expression::QExpression(exprlist) => {
				2 + exprlist.iter().map(Self::lexeme_len).sum::<usize>()
			}
			Expression::Atom(_) => 1,
			Expression::Null => 2,
		}
	}

	pub fn parse_string_literal(s: &str) -> Result<String, LockjawParseError> {
		let mut iter = s.chars();
		let mut escaped = String::with_capacity(s.len());
		while let Some(c) = iter.next() {
			escaped.push(match c {
				'\\' => match iter.next() {
					Some(escape) => match escape {
						't' => '\t',
						'n' => '\n',
						'r' => '\r',
						'0' => '\0',
						'\\' => '\\',
						'"' => '"',
						'\'' => '\'',
						_ => return Err(LockjawParseError::InvalidStringLiteral { code: escape }),
					},
					None => return Err(LockjawParseError::UnexpectedEof),
				},
				_ => c,
			})
		}
		Ok(escaped)
	}

	pub fn parse_from_text(s: &str) -> Result<Expression, LockjawParseError> {
		let lexemes: Result<Vec<lexer::Lexeme>, LockjawParseError> = lexer::Lexer::new(s).collect();
		Self::parse(lexemes?.as_slice())
	}

	pub fn parse(lexemes: &[Lexeme]) -> Result<Expression, LockjawParseError> {
		match lexemes[0].value {
			LexemeType::LeftParen => {
				let mut exprlist = VecDeque::new();
				let mut current_lexeme = 1;
				while current_lexeme < lexemes.len()
					&& LexemeType::RightParen != lexemes[current_lexeme].value
				{
					let expression = Self::parse(&lexemes[current_lexeme..])?;
					current_lexeme += Self::lexeme_len(&expression);
					exprlist.push_back(expression);
				}
				Ok(Expression::SExpression(exprlist))
			}
			LexemeType::LeftCBracket => {
				let mut exprlist = VecDeque::new();
				let mut current_lexeme = 1;
				while current_lexeme < lexemes.len()
					&& LexemeType::RightCBracket != lexemes[current_lexeme].value
				{
					let expression = Self::parse(&lexemes[current_lexeme..])?;
					current_lexeme += Self::lexeme_len(&expression);
					exprlist.push_back(expression);
				}
				Ok(Expression::QExpression(exprlist))
			}
			term => Ok(Expression::Atom(match term {
				LexemeType::Integer(value) => Atom::Number(Numeric::Int(value)),
				LexemeType::Float(value) => Atom::Number(Numeric::Float(value)),
				LexemeType::StringLiteral(str) => Atom::String(Self::parse_string_literal(str)?),
				LexemeType::RawSymbol(symb) => Atom::Symbol(symb.to_string()),
				_ => {
					return Err(LockjawParseError::InvalidLiteral {
						index: lexemes[0].index,
					})
				}
			})),
		}
	}

	pub fn parse_root(lexemes: &[Lexeme]) -> Result<Expression, LockjawParseError> {
		let mut expressions = VecDeque::new();
		let mut lexemes_consumed = 0;
		while lexemes_consumed < lexemes.len() {
			let expression = Self::parse(&lexemes[lexemes_consumed..])?;
			lexemes_consumed += Self::lexeme_len(&expression);
			expressions.push_back(expression);
		}

		Ok(Expression::SExpression(expressions))
	}
}
