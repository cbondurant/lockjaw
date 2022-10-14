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
		// get from local env if exists, otherwise check global
		// Decided against checking intermediate values since currying
		// Can be used to import the immediate environment
		self.internal[self.internal.len() - 1]
			.get(k.as_str())
			.or(self.internal[0].get(k.as_str()))
	}

	pub fn push_env(&mut self) {
		self.internal.push(HashMap::new())
	}

	pub fn pop_env(&mut self) -> Option<HashMap<String, Value>> {
		self.internal.pop()
	}
}
