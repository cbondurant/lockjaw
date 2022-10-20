//#![allow(dead_code)]
mod builtins;
mod environment;
mod evaluator;
mod lexer;
mod numeric;
mod parser;
mod types;

use std::path::PathBuf;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(clap::Parser)]
#[command(name = "lockjaw")]
#[command(author = "Conner Bondurant")]
struct Cli {
	file: Option<PathBuf>,
	#[arg(short, long)]
	load_to_interpreter: bool,
}

fn main() {
	let cli = Cli::parse();

	let mut environment = evaluator::Evaluator::new();
	if let Some(run_program) = cli.file.as_deref() {
		let program = format!("load \"{}\"", run_program.display());
		println!("{}", program);
		let lexemes: Result<Vec<lexer::Lexeme>, parser::LockjawParseError> =
			lexer::Lexer::new(program.as_str()).collect();
		match lexemes {
			Ok(lexemes) => match parser::Parser::parse_root(lexemes.as_slice()) {
				Ok(lj) => {
					//println!("{:#?}", lj);
					println!("{:?}", environment.evaluate(lj));
				}
				Err(parser_err) => {
					println!("{parser_err:?}");
					match parser_err {
						parser::LockjawParseError::InvalidLiteral { index } => {
							println!("{}^", " ".to_string().repeat(index))
						}
						parser::LockjawParseError::UnexpectedEof => {
							println!("Unexpected EOF!")
						}
						parser::LockjawParseError::InvalidStringLiteral { code } => {
							println!("Invalid escape code: {}", code)
						}
					}
				}
			},
			Err(why) => println!("{why:?}"),
		}
		if !cli.load_to_interpreter {
			return;
		}
	}

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
				let lexemes: Result<Vec<lexer::Lexeme>, parser::LockjawParseError> =
					lexer::Lexer::new(&line).collect();
				match lexemes {
					Ok(lexemes) => match parser::Parser::parse_root(lexemes.as_slice()) {
						Ok(lj) => {
							//println!("{:#?}", lj);
							println!("{:?}", environment.evaluate(lj));
						}
						Err(parser_err) => {
							println!("{parser_err:?}");
							println!("{line}");
							match parser_err {
								parser::LockjawParseError::InvalidLiteral { index } => {
									println!("{}^", " ".to_string().repeat(index))
								}
								parser::LockjawParseError::UnexpectedEof => {
									println!("Unexpected EOF!")
								}
								parser::LockjawParseError::InvalidStringLiteral { code } => {
									println!("Invalid escape code: {}", code)
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

	#[test]
	fn functions_define_properly() {
		assert_program_output(
			vec!["def {inc} (fun {x} {+ x 1})", "inc 3"],
			Expression::Atom(Atom::Number(Numeric::Int(4))),
		);
	}

	#[test]
	fn functions_can_call_functions() {
		assert_program_output(
			vec!["def {square} (fun {x} {* x x})", "square 4"],
			Expression::Atom(Atom::Number(Numeric::Int(16))),
		)
	}

	#[test]
	fn unbound_curried_functions_evaluate() {
		assert_program_output(
			vec!["def {two_args} (fun {x y} {* y x})", "(two_args 2) 2"],
			Expression::Atom(Atom::Number(Numeric::Int(4))),
		)
	}

	#[test]
	fn can_recurse_using_cond() {
		assert_program_output(
			vec![
				"def {lat?}
				(fun {l}
					{cond
						{(null? l) #t}
						{(atom? (car l)) (lat? (cdr l))}
						{else #f}})",
				"lat? {1 2 3 4 5 6 7 8 9 0}",
			],
			Expression::Atom(Atom::Bool(true)),
		)
	}

	#[test]
	fn boolean_expressions_confirm() {
		assert_program_output(vec!["and? #t #t"], Expression::Atom(Atom::Bool(true)));
		assert_program_output(vec!["and? #t #f"], Expression::Atom(Atom::Bool(false)));
		assert_program_output(vec!["and? #f #t"], Expression::Atom(Atom::Bool(false)));
		assert_program_output(vec!["and? #f #f"], Expression::Atom(Atom::Bool(false)));
		assert_program_output(vec!["or? #t #t"], Expression::Atom(Atom::Bool(true)));
		assert_program_output(vec!["or? #t #f"], Expression::Atom(Atom::Bool(true)));
		assert_program_output(vec!["or? #f #t"], Expression::Atom(Atom::Bool(true)));
		assert_program_output(vec!["or? #f #f"], Expression::Atom(Atom::Bool(false)));
		assert_program_output(vec!["xor? #t #t"], Expression::Atom(Atom::Bool(false)));
		assert_program_output(vec!["xor? #t #f"], Expression::Atom(Atom::Bool(true)));
		assert_program_output(vec!["xor? #f #t"], Expression::Atom(Atom::Bool(true)));
		assert_program_output(vec!["xor? #f #f"], Expression::Atom(Atom::Bool(false)));
	}

	#[test]
	fn weird_string_literals_pass() {
		assert_program_output(
			vec!["\"!@#$%^&*()_+<>,.;':\\\"\\n\\t\\r`~😇👩👩👧👦🌓🌑🏳️⚧️\""],
			Expression::Atom(Atom::String(String::from(
				"!@#$%^&*()_+<>,.;':\"\n\t\r`~😇👩👩👧👦🌓🌑🏳️⚧️",
			))),
		)
	}
}
