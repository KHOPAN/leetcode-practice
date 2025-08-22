impl Solution {
	pub fn find_median_sorted_arrays(first: Vec<i32>, second: Vec<i32>) -> f64 {
		let first_length : usize = first.len();
		let second_length : usize = second.len();
		let total_length : usize = first_length + second_length;
		let mut first_pointer : usize = 0;
		let mut second_pointer : usize = 0;
		let mut previous_value : i32 = 0;

		while first_pointer < first_length || second_pointer < second_length {
			let index : usize = first_pointer + second_pointer;
			let value : i32;

			if first_pointer < first_length && (second_pointer >= second_length || first[first_pointer] < second[second_pointer]) {
				value = first[first_pointer];
				first_pointer += 1;
			} else {
				value = second[second_pointer];
				second_pointer += 1;
			}

			if index == total_length / 2 {
				return if total_length % 2 == 0 { (value + previous_value) as f64 / 2.0f64 } else { value as f64 };
			}

			previous_value = value;
		}

		0.0f64
	}
}
