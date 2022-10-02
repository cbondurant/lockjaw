#![allow(dead_code)]
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
				rl.add_history_entry(line.as_str());
				let lexemes = lexer::Lexer::new(&line).collect();
				match parser::Lockjaw::parse(lexemes) {
					Ok(lj) => println!("{:?}", lj),
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
