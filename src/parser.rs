use crate::lexer::{self, LexingError};
use std::{
	collections::VecDeque,
	error::Error,
	fmt::Display,
	num::{ParseFloatError, ParseIntError},
	str::FromStr,
};

use crate::{
	lexer::{Lexeme, LexemeType},
	numeric::Numeric,
	types::*,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsingError {
	InvalidLiteral { index: usize },
	InvalidStringLiteral { code: char },
	LexingError(LexingError),
	IntParseFailure(<i64 as FromStr>::Err),
	FloatParseFailure(<f64 as FromStr>::Err),
	UnexpectedEof,
}

impl Display for ParsingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParsingError::InvalidLiteral { index } => write!(f, "invalid literal at index {index}"),
			ParsingError::InvalidStringLiteral { code } => {
				write!(f, "invalid character in string literal: {code}")
			}
			ParsingError::LexingError(_) => write!(f, "failure at lexing stage"),
			ParsingError::UnexpectedEof => {
				write!(f, "reached end of file before end of string literal")
			}
			ParsingError::IntParseFailure(_) => write!(f, "failed to parse integer literal"),
			ParsingError::FloatParseFailure(_) => write!(f, "failed to parse float literal"),
		}
	}
}

impl From<ParseFloatError> for ParsingError {
	fn from(e: <f64 as FromStr>::Err) -> Self {
		Self::FloatParseFailure(e)
	}
}

impl From<ParseIntError> for ParsingError {
	fn from(e: <i64 as FromStr>::Err) -> Self {
		Self::IntParseFailure(e)
	}
}

impl From<LexingError> for ParsingError {
	fn from(e: LexingError) -> Self {
		Self::LexingError(e)
	}
}

impl Error for ParsingError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		if let ParsingError::LexingError(lex_error) = self {
			Some(lex_error)
		} else {
			None
		}
	}
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

	pub fn parse_string_literal(s: &str) -> Result<String, ParsingError> {
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
						_ => return Err(ParsingError::InvalidStringLiteral { code: escape }),
					},
					None => return Err(ParsingError::UnexpectedEof),
				},
				_ => c,
			})
		}
		Ok(escaped)
	}

	pub fn parse_from_text(s: &str) -> Result<Expression, ParsingError> {
		let lexemes: Result<Vec<lexer::Lexeme>, LexingError> = lexer::Lexer::new(s).collect();
		Self::parse_root(lexemes?.as_slice())
	}

	pub fn parse(lexemes: &[Lexeme]) -> Result<Expression, ParsingError> {
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
				LexemeType::Integer(value) => Atom::Number(Numeric::Int(value.parse()?)),
				LexemeType::Float(value) => Atom::Number(Numeric::Float(value.parse()?)),
				LexemeType::StringLiteral(str) => Atom::String(Self::parse_string_literal(str)?),
				LexemeType::RawSymbol(symb) => Atom::Symbol(symb.to_string()),
				_ => {
					return Err(ParsingError::InvalidLiteral {
						index: lexemes[0].index,
					})
				}
			})),
		}
	}

	pub fn parse_root(lexemes: &[Lexeme]) -> Result<Expression, ParsingError> {
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
