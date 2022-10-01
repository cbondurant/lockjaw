use std::io::{self, Write};

fn main() {
	println!("Lockjaw Version 0.1.0");
	println!("Press Ctrl+c to Exit");

	let mut buff = String::new();
	let stdin = io::stdin();

	loop {
		print!("lj> ");
		io::stdout().flush().unwrap();

		if let Ok(_) = stdin.read_line(&mut buff) {
			println!("No! You're a {}", buff);
		}
	}
}
