impl Solution {
	pub fn zero_filled_subarray(numbers: Vec<i32>) -> i64 {
		let mut total : i64 = 0;
		let mut zeros : i64 = 0;

		for number in numbers {
			if number == 0 {
				zeros = zeros + 1;
				continue;
			}

			total += zeros * (zeros + 1) / 2;
			zeros = 0;
		}

		total + zeros * (zeros + 1) / 2
	}
}
