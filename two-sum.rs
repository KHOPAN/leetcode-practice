impl Solution {
	pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
		for x in 0..numbers.len() {
			let remaining = target - numbers[x];

			for y in (x + 1)..numbers.len() {
				if remaining - numbers[y] == 0 {
					return vec![x as i32, y as i32];
				}
			}
		}

		Vec::new()
	}
}
