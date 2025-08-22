impl Solution {
	pub fn length_of_longest_substring(text: String) -> i32 {
		let mut maximum : usize = 0;
		let mut list : Vec<char> = Vec::new();
		let mut left : usize = 0;

		for right in 0..text.len() {
			let character : char = text.chars().nth(right).unwrap();

			while list.contains(&character) {
				list.remove(list.iter().position(|&x| x == text.chars().nth(left).unwrap()).unwrap());
				left += 1;
			}

			list.insert(list.len(), character);
			maximum = std::cmp::max(maximum, right - left + 1);
		}

		maximum as i32
	}
}
