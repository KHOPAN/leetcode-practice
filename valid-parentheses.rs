impl Solution {
	pub fn is_valid(text: String) -> bool {
		let mut stack : Vec<i8> = Vec::new();

		for character in text.chars() {
			match character {
				'(' => stack.push(0),
				'[' => stack.push(1),
				'{' => stack.push(2),
				')' => if stack.pop().unwrap_or(-1) != 0 { return false; },
				']' => if stack.pop().unwrap_or(-1) != 1 { return false; },
				'}' => if stack.pop().unwrap_or(-1) != 2 { return false; },
				_ => return false,
			}
		}

		stack.is_empty()
	}
}
