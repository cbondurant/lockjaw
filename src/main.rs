#![allow(dead_code)]
mod evaluator;
mod lexer;
mod parser;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
	println!("Lockjaw Version 0.1.1");
	println!("Press Ctrl+c to Exit");

	let mut rl = match Editor::<()>::new() {
		Ok(rl) => rl,
		Err(why) => {
			println!("Error creating prompt: {}", why);
			return;
		}
	};

	loop {
		let readline = rl.readline("lj> ");
		match readline {
			Ok(line) => {
				if line.is_empty() {
					continue;
				};
				rl.add_history_entry(line.as_str());
				let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&line).collect();
				match parser::Expression::parse_root(lexemes.as_slice()) {
					Ok(lj) => {
						println!("{:#?}", lexemes);
						println!("{:#?}", lj);
						println!("{:#?}", evaluator::Evaluator::evaluate(&lj));
					}
					Err(parser_err) => {
						println!("{parser_err:?}");
						println!("{line}");
						match parser_err {
							parser::LockjawParseError::InvalidOperator { index } => {
								println!("{}^", " ".to_string().repeat(index))
							}
							parser::LockjawParseError::InvalidLiteral { index } => {
								println!("{}^", " ".to_string().repeat(index))
							}
						}
					}
				}
			}
			Err(ReadlineError::Interrupted) => {
				println!("CTRL+C! Closing.");
				break;
			}
			Err(ReadlineError::Eof) => {
				println!("EOF");
				break;
			}
			Err(why) => {
				println!("Unexpected Read Err: {:?}", why);
				break;
			}
		}
	}
}
