#[derive(Debug)]
pub enum Lexeme {
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
			println!("{}", nextchar == '.');
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
					Some(Lexeme::Float(
						self.text[numstart..numstart + num_split + float_end]
							.parse()
							.unwrap(),
					))
				} else {
					self.index = self.text.len();
					Some(Lexeme::Float(self.text[numstart..].parse().unwrap()))
				}
			} else {
				Some(Lexeme::Integer(
					self.text[numstart..numstart + num_split].parse().unwrap(),
				))
			}
		} else {
			self.index = self.text.len();
			Some(Lexeme::Integer(self.text[numstart..].parse().unwrap()))
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Lexeme;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(char) = self.advance_char() {
			let resp = match char {
				'(' => Some(Lexeme::LeftParen),
				')' => Some(Lexeme::RightParen),
				'\'' => Some(Lexeme::SingleQuote),
				'"' => Some(Lexeme::DoubleQuote),
				'+' => Some(Lexeme::Plus),
				'-' => Some(Lexeme::Dash),
				'*' => Some(Lexeme::Asterisk),
				'/' => Some(Lexeme::ForwardSlash),
				'0'..='9' => self.lex_number(),
				' ' | '\t' => continue,
				_ => panic!("Invalid Syntax!"),
			};
			return resp;
		}
		None
	}
}
