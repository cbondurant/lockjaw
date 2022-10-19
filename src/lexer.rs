use crate::parser::LockjawParseError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexemeType<'a> {
	RightParen,
	LeftParen,
	RightCBracket,
	LeftCBracket,
	Integer(i64),
	Float(f64),
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

	fn lex_number(&mut self) -> LexemeType<'a> {
		let numstart = self.index - 1;
		if let Some((num_split, nextchar)) = self.text[numstart..]
			.chars()
			.enumerate()
			.find(|(_, x)| !x.is_ascii_digit())
		{
			if nextchar == '.' {
				if let Some((float_end, _)) = self.text[numstart + num_split + 1..]
					.chars()
					.enumerate()
					.find(|(_, x)| !x.is_ascii_digit())
				{
					if float_end == 0 {
						panic!("Invalid Literal!");
					}
					self.index += num_split + float_end;
					LexemeType::Float(
						self.text[numstart..numstart + num_split + float_end]
							.parse()
							.unwrap(),
					)
				} else {
					self.index = self.text.len();
					LexemeType::Float(self.text[numstart..].parse().unwrap())
				}
			} else {
				self.index = numstart + num_split;
				LexemeType::Integer(self.text[numstart..numstart + num_split].parse().unwrap())
			}
		} else {
			self.index = self.text.len();
			LexemeType::Integer(self.text[numstart..].parse().unwrap())
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

	fn lex_string_literal(&mut self) -> Result<LexemeType<'a>, LockjawParseError> {
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
		Err(LockjawParseError::UnexpectedEof)
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Lexeme<'a>, LockjawParseError>;

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
					'0'..='9' => self.lex_number(),
					' ' | '\t' | '\n' => continue,
					x if Self::is_valid_raw_symbol(x) => self.lex_raw_symbol(),
					_ => return Some(Err(LockjawParseError::InvalidLiteral { index })),
				},
			}));
			return resp;
		}
		None
	}
}
