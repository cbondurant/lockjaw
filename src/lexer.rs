#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexemeType {
	RightParen,
	LeftParen,
	SingleQuote,
	DoubleQuote,
	Plus,
	Dash,
	ForwardSlash,
	Asterisk,
	Integer(i64),
	Float(f64),
}

#[derive(Debug, Clone, Copy)]
pub struct Lexeme {
	pub index: usize,
	pub value: LexemeType,
}

impl Lexeme {
	pub fn value(&self) -> LexemeType {
		self.value
	}

	pub fn index(&self) -> usize {
		self.index
	}
}

pub struct Lexer<'a> {
	text: &'a str,
	index: usize,
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> Self {
		Lexer { text, index: 0 }
	}

	fn advance_char(&mut self) -> Option<char> {
		self.index += 1;
		self.text[self.index - 1..].chars().next()
	}

	fn lex_number(&mut self) -> Option<Lexeme> {
		let numstart = self.index - 1;
		if let Some((num_split, nextchar)) = self.text[numstart..]
			.chars()
			.enumerate()
			.find(|(_, x)| !x.is_numeric())
		{
			if nextchar == '.' {
				if let Some((float_end, _)) = self.text[numstart + num_split + 1..]
					.chars()
					.enumerate()
					.find(|(_, x)| !x.is_numeric())
				{
					if float_end == 0 {
						panic!("Invalid Literal!");
					}
					self.index += num_split + float_end;
					Some(Lexeme {
						index: numstart,
						value: LexemeType::Float(
							self.text[numstart..numstart + num_split + float_end]
								.parse()
								.unwrap(),
						),
					})
				} else {
					self.index = self.text.len();
					Some(Lexeme {
						index: numstart,
						value: LexemeType::Float(self.text[numstart..].parse().unwrap()),
					})
				}
			} else {
				Some(Lexeme {
					index: numstart,
					value: LexemeType::Integer(
						self.text[numstart..numstart + num_split].parse().unwrap(),
					),
				})
			}
		} else {
			self.index = self.text.len();
			Some(Lexeme {
				index: numstart,
				value: LexemeType::Integer(self.text[numstart..].parse().unwrap()),
			})
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Lexeme;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(char) = self.advance_char() {
			let index = self.index - 1;
			let resp = match char {
				'(' => Some(Lexeme {
					index,
					value: LexemeType::LeftParen,
				}),
				')' => Some(Lexeme {
					index,
					value: LexemeType::RightParen,
				}),
				'\'' => Some(Lexeme {
					index,
					value: LexemeType::SingleQuote,
				}),
				'"' => Some(Lexeme {
					index,
					value: LexemeType::DoubleQuote,
				}),
				'+' => Some(Lexeme {
					index,
					value: LexemeType::Plus,
				}),
				'-' => Some(Lexeme {
					index,
					value: LexemeType::Dash,
				}),
				'*' => Some(Lexeme {
					index,
					value: LexemeType::Asterisk,
				}),
				'/' => Some(Lexeme {
					index,
					value: LexemeType::ForwardSlash,
				}),
				'0'..='9' => self.lex_number(),
				' ' | '\t' => continue,
				_ => panic!("Invalid Syntax!"),
			};
			return resp;
		}
		None
	}
}
