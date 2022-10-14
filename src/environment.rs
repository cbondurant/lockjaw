use crate::types::Value;
use std::collections::HashMap;

pub struct Environment {
	internal: Vec<HashMap<String, Value>>,
}

impl Environment {
	pub fn new() -> Environment {
		Environment {
			internal: vec![HashMap::new()],
		}
	}

	pub fn put(&mut self, k: String, v: Value) {
		let len = self.internal.len();
		self.internal[len - 1].insert(k, v);
	}

	pub fn def(&mut self, k: String, v: Value) {
		self.internal[0].insert(k, v);
	}

	pub fn get(&self, k: String) -> Option<&Value> {
		// Starting at the end (newest environment), look for the first instance of key
		self.internal
			.iter()
			.rev()
			.find_map(|env| env.get(k.as_str()))
	}

	pub fn push_env(&mut self) {
		self.internal.push(HashMap::new())
	}

	pub fn pop_env(&mut self) -> Option<HashMap<String, Value>> {
		self.internal.pop()
	}
}
