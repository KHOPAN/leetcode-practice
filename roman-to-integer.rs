impl Solution {
	pub fn roman_to_int(string: String) -> i32 {
		let mut result : i32 = 0;
		let mut last_increment : u16 = 0;

		for character in string.chars().rev() {
			if character == 'I' {
				if last_increment == 5 || last_increment == 10 {
					last_increment = 0;
					result -= 1;
					continue;
				}

				last_increment = 1;
			} else if character == 'V' {
				last_increment = 5;
			} else if character == 'X' {
				if last_increment == 50 || last_increment == 100 {
					last_increment = 0;
					result -= 10;
					continue;
				}

				last_increment = 10;
			} else if character == 'L' {
				last_increment = 50;
			} else if character == 'C' {
				if last_increment == 500 || last_increment == 1000 {
					last_increment = 0;
					result -= 100;
					continue;
				}

				last_increment = 100;
			} else if character == 'D' {
				last_increment = 500;
			} else if character == 'M' {
				last_increment = 1000;
			} else {
				last_increment = 0;
			}

			result += last_increment as i32;
		}

		result
	}
}
