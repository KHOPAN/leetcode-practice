impl Solution {
	pub fn maximum_triplet_value(numbers: Vec<i32>) -> i64 {
		let length : usize = numbers.len();
		let mut maximum : i64 = 0;

		for first in 0..length {
			for second in first + 1..length {
				for third in second + 1..length {
					maximum = maximum.max((numbers[first] as i64 - numbers[second] as i64) * numbers[third] as i64);
				}
			}
		}

		maximum
	}
}
