impl Solution {
	pub fn three_consecutive_odds(array: Vec<i32>) -> bool {
		if array.len() < 3 {
			return false;
		}

		let mut first : i32 = array[0];
		let mut second : i32 = array[1];

		for i in 2..array.len() {
			if first % 2 == 1 && second % 2 == 1 && array[i] % 2 == 1 {
				return true;
			}

			first = second;
			second = array[i];
		}

		false
	}
}
