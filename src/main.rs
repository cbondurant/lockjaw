//#![allow(dead_code)]
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

	let environment = evaluator::Evaluator::new();

	loop {
		let readline = rl.readline("lj> ");
		match readline {
			Ok(line) => {
				rl.add_history_entry(line.as_str());
				let lexemes: Result<Vec<lexer::Lexeme>, parser::LockjawParseError> =
					lexer::Lexer::new(&line).collect();
				match lexemes {
					Ok(lexemes) => match parser::Expression::parse_root(lexemes.as_slice()) {
						Ok(lj) => {
							println!("{:#?}", lj);
							println!("{:?}", environment.evaluate(lj));
						}
						Err(parser_err) => {
							println!("{parser_err:?}");
							println!("{line}");
							match parser_err {
								parser::LockjawParseError::InvalidLiteral { index } => {
									println!("{}^", " ".to_string().repeat(index))
								}
							}
						}
					},
					Err(why) => println!("{why:?}"),
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

#[cfg(test)]
mod tests {

	use crate::evaluator;
	use crate::lexer;
	use crate::parser;
	use crate::parser::{Atom, Expression};

	#[test]
	fn plus_adds() {
		let environment = evaluator::Evaluator::new();
		let command = "+ 3 4";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		if let Expression::Atom(Atom::Int(x)) = result {
			assert_eq!(x, 7);
		}
	}

	#[test]
	fn minus_subtracts() {
		let environment = evaluator::Evaluator::new();
		let command = "- 3 1 1 1 ";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		if let Expression::Atom(Atom::Int(x)) = result {
			assert_eq!(x, 0);
		}
	}

	#[test]
	fn minus_negates() {
		let environment = evaluator::Evaluator::new();
		let command = "- 1";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		if let Expression::Atom(Atom::Int(x)) = result {
			assert_eq!(x, -1);
		}
	}

	#[test]
	fn math_operations_upcast_to_float() {
		let environment = evaluator::Evaluator::new();
		let command = "+ 1 2.4";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		let m = matches!(result, Expression::Atom(Atom::Float(_)));
		assert!(m);

		let environment = evaluator::Evaluator::new();
		let command = "- 1 2.4";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		let m = matches!(result, Expression::Atom(Atom::Float(_)));
		assert!(m);

		let environment = evaluator::Evaluator::new();
		let command = "* 1 2.4";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		let m = matches!(result, Expression::Atom(Atom::Float(_)));
		assert!(m);

		let environment = evaluator::Evaluator::new();
		let command = "/ 1 2";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		let m = matches!(result, Expression::Atom(Atom::Float(_)));
		assert!(m);
	}

	#[test]
	fn quote_handles_valid_expressions() {
		let environment = evaluator::Evaluator::new();
		let command = "quote 1 2 4 2 + - * \\ / 34 dsfgsd 345 &";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		let m = matches!(result, Expression::QExpression(_));
		assert!(m);
	}

	#[test]
	fn curly_brackets_quote() {
		let environment = evaluator::Evaluator::new();
		let command = "eval {+ 1 2 3}";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		let m = matches!(result, Expression::Atom(Atom::Int(6)));
		assert!(m);
	}

	#[test]
	fn car_gets_front_element_of_qexpr() {
		let environment = evaluator::Evaluator::new();
		let command = "car {+ 1 2 3}";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		let m = matches!(result, Expression::Atom(Atom::Symbol("+")));
		assert!(m);
	}

	#[test]
	fn cdr_gets_tail_of_qexpr() {
		let environment = evaluator::Evaluator::new();
		let command = "cdr {+ 1 }";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		if let Expression::QExpression(v) = result {
			match v.get(0) {
				Some(atom) => assert!(matches!(atom, &Expression::Atom(Atom::Int(1)))),
				None => assert!(false),
			}
		} else {
			assert!(false)
		}
	}

	#[test]
	fn join_combines_qexprs() {
		let environment = evaluator::Evaluator::new();
		let command = "eval (join {+} {1 2 3})";
		let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
			.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
			.unwrap();
		let parse = parser::Expression::parse_root(lexemes.as_slice()).unwrap();
		let result = environment.evaluate(parse).unwrap();
		let m = matches!(result, Expression::Atom(Atom::Int(6)));
		assert!(m);
	}
}
