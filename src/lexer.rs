#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexemeType<'a> {
	RightParen,
	LeftParen,
	RightCBracket,
	LeftCBracket,
	SingleQuote,
	DoubleQuote,
	Plus,
	Dash,
	ForwardSlash,
	Asterisk,
	Integer(i64),
	Float(f64),
	RawSymbol(&'a str),
	Err,
}

#[derive(Debug, Clone, Copy)]
pub struct Lexeme<'a> {
	pub index: usize,
	pub value: LexemeType<'a>,
}

impl<'a> Lexeme<'a> {
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

	fn lex_raw_symbol(&mut self) -> LexemeType<'a> {
		let symbol_start = self.index - 1;
		LexemeType::RawSymbol(
			&self.text[symbol_start
				..if let Some(i) = self.text[self.index - 1..].find(char::is_whitespace) {
					self.index = symbol_start + i;
					symbol_start + i
				} else {
					self.index = self.text.len();
					self.text.len()
				}],
		)
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Lexeme<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(char) = self.advance_char() {
			let index = self.index - 1;
			let resp = Some(Lexeme {
				index,
				value: match char {
					'(' => LexemeType::LeftParen,
					')' => LexemeType::RightParen,
					'\'' => LexemeType::SingleQuote,
					'"' => LexemeType::DoubleQuote,
					'+' => LexemeType::Plus,
					'-' => LexemeType::Dash,
					'*' => LexemeType::Asterisk,
					'/' => LexemeType::ForwardSlash,
					'{' => LexemeType::LeftCBracket,
					'}' => LexemeType::RightCBracket,
					'0'..='9' => self.lex_number(),
					' ' | '\t' => continue,
					_ => self.lex_raw_symbol(),
				},
			});
			println!("{:#?}", resp);
			return resp;
		}
		None
	}
}
