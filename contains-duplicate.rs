impl Solution {
	pub fn contains_duplicate(mut numbers: Vec<i32>) -> bool {
		if numbers.len() <= 1 {
			return false;
		}

		numbers.sort();
		let mut previous : i32 = numbers[0];

		for i in 1..numbers.len() {
			if numbers[i] == previous {
				return true;
			}

			previous = numbers[i];
		}

		false
	}
}
