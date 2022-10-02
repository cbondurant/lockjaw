mod lexer;

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
				for lexeme in lexer::Lexer::new(&line) {
					println!("{:?}", lexeme);
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
