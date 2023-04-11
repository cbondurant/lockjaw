use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexingErrorKind {
	InvalidLiteral { expected: String, got: String },
	UnexpectedEof,
}

impl Display for LexingErrorKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			LexingErrorKind::InvalidLiteral { expected, got } => {
				write!(f, "expected `{expected}`, got `{got}`")
			}
			LexingErrorKind::UnexpectedEof => write!(f, "unexpected end of file"),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexingError {
	position: usize,
	kind: LexingErrorKind,
}

impl Display for LexingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Error at position {}: {}", self.position, self.kind)
	}
}

impl Error for LexingError {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexemeType<'a> {
	RightParen,
	LeftParen,
	RightCBracket,
	LeftCBracket,
	// For better seperation of responsibility, we only grab the string rep
	// Parsing them into integers and floats is for the parser.
	Integer(&'a str),
	Float(&'a str),
	RawSymbol(&'a str),
	StringLiteral(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub struct Lexeme<'a> {
	pub index: usize,
	pub value: LexemeType<'a>,
}

impl<'a> Lexeme<'a> {}

pub struct Lexer<'a> {
	text: &'a str,
	index: usize,
	has_errored: bool,
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> Self {
		Lexer {
			text,
			index: 0,
			has_errored: false,
		}
	}

	fn advance_char(&mut self) -> Option<char> {
		self.index += 1;
		self.text[self.index - 1..].chars().next()
	}

	fn lex_number(&mut self) -> Result<LexemeType<'a>, LexingError> {
		let numstart = self.index - 1;
		if let Some((num_split, nextchar)) = self.text[numstart..]
			.chars()
			.enumerate()
			.find(|(_, x)| !x.is_ascii_digit())
		{
			if nextchar == '.' {
				if let Some((float_end, _)) = self.text[numstart + num_split + 1..]
					.char_indices()
					.find(|(_, x)| !x.is_ascii_digit())
				{
					self.index += num_split + float_end + 1;
					Ok(LexemeType::Float(
						&self.text[numstart..numstart + num_split + float_end + 1],
					))
				} else {
					self.index = self.text.len();
					Ok(LexemeType::Float(&self.text[numstart..]))
				}
			} else {
				self.index = numstart + num_split;
				Ok(LexemeType::Integer(
					&self.text[numstart..numstart + num_split],
				))
			}
		} else {
			self.index = self.text.len();
			Ok(LexemeType::Integer(&self.text[numstart..]))
		}
	}

	fn is_valid_raw_symbol(c: char) -> bool {
		match c {
			'a'..='z' => true,
			'A'..='Z' => true,
			// '0'..='9' => true, // Might turn on eventually, not yet.
			'/' | '_' | '+' | '-' | '*' | '\\' | '=' | '>' | '<' | '!' | '&' | '?' | '#' => true,
			_ => false,
		}
	}

	fn lex_raw_symbol(&mut self) -> LexemeType<'a> {
		let symbol_start = self.index - 1;
		LexemeType::RawSymbol(
			&self.text[symbol_start
				..if let Some(i) =
					self.text[self.index - 1..].find(|x| !Self::is_valid_raw_symbol(x))
				{
					self.index = symbol_start + i;
					symbol_start + i
				} else {
					self.index = self.text.len();
					self.text.len()
				}],
		)
	}

	fn lex_string_literal(&mut self) -> Result<LexemeType<'a>, LexingError> {
		let symbol_start = self.index - 1;
		let mut forward_iter = self.text[symbol_start..].char_indices();
		let (_, start_char) = forward_iter.next().unwrap();
		while let Some((i, c)) = forward_iter.next() {
			if c == '\\' {
				forward_iter.next().unwrap();
			} else if c == start_char {
				self.index = symbol_start + i + 1;
				return Ok(LexemeType::StringLiteral(
					&self.text[symbol_start + 1..symbol_start + i],
				));
			}
		}
		Err(LexingError {
			position: self.index,
			kind: LexingErrorKind::UnexpectedEof,
		})
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Lexeme<'a>, LexingError>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.has_errored {
			return None;
		}

		while let Some(char) = self.advance_char() {
			let index = self.index - 1;
			let resp = Some(Ok(Lexeme {
				index,
				value: match char {
					'(' => LexemeType::LeftParen,
					')' => LexemeType::RightParen,
					'{' => LexemeType::LeftCBracket,
					'}' => LexemeType::RightCBracket,
					';' => {
						self.index += self.text[self.index..]
							.find('\n')
							.unwrap_or(self.text.len() - self.index);
						continue;
					}
					'"' | '\'' => match self.lex_string_literal() {
						Ok(val) => val,
						Err(e) => return Some(Err(e)),
					},
					'0'..='9' => match self.lex_number() {
						Ok(val) => val,
						Err(e) => return Some(Err(e)),
					},
					' ' | '\t' | '\n' => continue,
					x if Self::is_valid_raw_symbol(x) => self.lex_raw_symbol(),
					invalid => {
						return Some(Err(LexingError {
							position: index,
							kind: LexingErrorKind::InvalidLiteral {
								expected: String::from(
									"one of '(', ')', '{', '}', ';' '\"', '\'', ",
								),
								got: invalid.to_string(),
							},
						}))
					}
				},
			}));
			return resp;
		}
		None
	}
}
