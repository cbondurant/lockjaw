//#![allow(dead_code)]
mod builtins;
mod environment;
mod evaluator;
mod lexer;
mod numeric;
mod parser;
mod types;

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

	let mut environment = evaluator::Evaluator::new();

	loop {
		let readline = rl.readline("lj> ");
		match readline {
			Ok(line) => {
				rl.add_history_entry(line.as_str());
				let lexemes: Result<Vec<lexer::Lexeme>, parser::LockjawParseError> =
					lexer::Lexer::new(&line).collect();
				match lexemes {
					Ok(lexemes) => match parser::Parser::parse_root(lexemes.as_slice()) {
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

	use std::collections::VecDeque;

	use crate::evaluator;
	use crate::lexer;
	use crate::numeric::Numeric;
	use crate::parser;
	use crate::types::*;

	fn assert_program_output(commands: Vec<&str>, expected_output: Expression) {
		let mut environment = evaluator::Evaluator::new();
		let mut result: Expression = Expression::SExpression(VecDeque::new());
		for command in commands {
			let lexemes: Vec<lexer::Lexeme> = lexer::Lexer::new(&command)
				.collect::<Result<Vec<lexer::Lexeme>, parser::LockjawParseError>>()
				.unwrap();
			let parse = parser::Parser::parse_root(lexemes.as_slice()).unwrap();
			result = environment.evaluate(parse).unwrap();
		}
		assert_eq!(expected_output, result);
	}

	#[test]
	fn plus_adds() {
		assert_program_output(
			vec!["+ 3 4"],
			Expression::Atom(Atom::Number(Numeric::Int(7))),
		);
	}

	#[test]
	fn plus_adds_variables() {
		assert_program_output(
			vec!["def {x} 3", "+ x 4"],
			Expression::Atom(Atom::Number(Numeric::Int(7))),
		);
	}

	#[test]
	fn minus_subtracts() {
		assert_program_output(
			vec!["- 3 1 1 1"],
			Expression::Atom(Atom::Number(Numeric::Int(0))),
		);
	}

	#[test]
	fn minus_negates() {
		assert_program_output(
			vec!["- 1"],
			Expression::Atom(Atom::Number(Numeric::Int(-1))),
		);
	}

	#[test]
	fn math_operations_upcast_to_float() {
		assert_program_output(
			vec!["+ 1 2.4"],
			Expression::Atom(Atom::Number(Numeric::Float(3.4))),
		);

		assert_program_output(
			vec!["- 1 2.4"],
			Expression::Atom(Atom::Number(Numeric::Float(-1.4))),
		);

		assert_program_output(
			vec!["* 1 2.4"],
			Expression::Atom(Atom::Number(Numeric::Float(2.4))),
		);

		assert_program_output(
			vec!["/ 1 2"],
			Expression::Atom(Atom::Number(Numeric::Float(0.5))),
		);
	}

	#[test]
	fn quote_handles_valid_expressions() {
		assert_program_output(
			vec!["quote 1 2 4 2 + - * \\ / dsfgsd &"],
			Expression::QExpression(VecDeque::from([
				Expression::Atom(Atom::Number(Numeric::Int(1))),
				Expression::Atom(Atom::Number(Numeric::Int(2))),
				Expression::Atom(Atom::Number(Numeric::Int(4))),
				Expression::Atom(Atom::Number(Numeric::Int(2))),
				Expression::Atom(Atom::Symbol(String::from("+"))),
				Expression::Atom(Atom::Symbol(String::from("-"))),
				Expression::Atom(Atom::Symbol(String::from("*"))),
				Expression::Atom(Atom::Symbol(String::from("\\"))),
				Expression::Atom(Atom::Symbol(String::from("/"))),
				Expression::Atom(Atom::Symbol(String::from("dsfgsd"))),
				Expression::Atom(Atom::Symbol(String::from("&"))),
			])),
		);
	}

	#[test]
	fn curly_brackets_quote() {
		assert_program_output(
			vec!["eval {+ 1 2 3}"],
			Expression::Atom(Atom::Number(Numeric::Int(6))),
		);
	}

	#[test]
	fn car_gets_front_element_of_qexpr() {
		assert_program_output(
			vec!["car {+ 1 2 3}"],
			Expression::Atom(Atom::Symbol(String::from("+"))),
		);
	}

	#[test]
	fn cdr_gets_tail_of_qexpr() {
		assert_program_output(
			vec!["cdr {+ 1 }"],
			Expression::QExpression(VecDeque::from([Expression::Atom(Atom::Number(
				Numeric::Int(1),
			))])),
		);
	}

	#[test]
	fn join_combines_qexprs() {
		assert_program_output(
			vec!["eval (join {+} {1 2 3})"],
			Expression::Atom(Atom::Number(Numeric::Int(6))),
		);
	}

	#[test]
	fn def_defines() {
		let commands = vec!["def {x} 3", "x"];
		assert_program_output(commands, Expression::Atom(Atom::Number(Numeric::Int(3))));
	}
}
