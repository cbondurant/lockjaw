use crate::lexer::{Lexeme, LexemeType};

#[derive(Debug)]
pub enum LockjawParseError {
	InvalidOperator { index: usize },
	InvalidLiteral { index: usize },
}

#[derive(Debug)]
pub enum Operator {
	Plus,
	Minus,
	Multiply,
	Divide,
}

impl Operator {
	pub fn parse(lexeme: Lexeme) -> Result<Self, LockjawParseError> {
		match lexeme {
			Lexeme {
				index: _,
				value: LexemeType::Plus,
			} => Ok(Self::Plus),
			Lexeme {
				index: _,
				value: LexemeType::Dash,
			} => Ok(Self::Minus),
			Lexeme {
				index: _,
				value: LexemeType::Asterisk,
			} => Ok(Self::Multiply),
			Lexeme {
				index: _,
				value: LexemeType::ForwardSlash,
			} => Ok(Self::Divide),
			Lexeme { index, value: _ } => Err(LockjawParseError::InvalidOperator { index }),
		}
	}
}

#[derive(Debug)]
pub enum NumberType {
	Float(f64),
	Int(i64),
}

#[derive(Debug)]
pub enum Expression {
	Number(NumberType),
	SExpression(Operator, Vec<Expression>),
}

impl Expression {
	pub fn lexeme_len(&self) -> usize {
		match self {
			// Open paren, Operator, <expr list> Close Paren.
			Self::SExpression(_, exprlist) => {
				3 + exprlist.iter().map(Expression::lexeme_len).sum::<usize>()
			}
			Self::Number(_) => 1,
		}
	}

	pub fn parse(lexemes: &[Lexeme]) -> Result<Self, LockjawParseError> {
		if LexemeType::LeftParen == lexemes[0].value() {
			let op = Operator::parse(lexemes[1])?;
			let mut exprlist = Vec::new();
			let mut current_lexeme = 2;
			while current_lexeme < lexemes.len()
				&& LexemeType::RightParen != lexemes[current_lexeme].value()
			{
				println!("{:?}", &lexemes[current_lexeme..]);
				let expression = Expression::parse(&lexemes[current_lexeme..])?;
				current_lexeme += expression.lexeme_len();
				exprlist.push(expression);
			}
			Ok(Expression::SExpression(op, exprlist))
		} else {
			match lexemes[0] {
				Lexeme {
					index: _,
					value: LexemeType::Integer(value),
				} => Ok(Expression::Number(NumberType::Int(value))),
				Lexeme {
					index: _,
					value: LexemeType::Float(value),
				} => Ok(Expression::Number(NumberType::Float(value))),
				Lexeme { index, value: _ } => Err(LockjawParseError::InvalidLiteral { index }),
			}
		}
	}
}

#[derive(Debug)]
pub struct Lockjaw {
	op: Operator,
	expressions: Vec<Expression>,
}

impl Lockjaw {
	pub fn parse(lexemes: Vec<Lexeme>) -> Result<Self, LockjawParseError> {
		let op = Operator::parse(lexemes[0])?;
		let mut expressions = Vec::new();
		let mut lexemes_consumed = 1;
		while lexemes_consumed < lexemes.len() {
			let expression = Expression::parse(&lexemes[lexemes_consumed..])?;
			lexemes_consumed += expression.lexeme_len();
			expressions.push(expression);
		}

		Ok(Lockjaw { op, expressions })
	}
}
