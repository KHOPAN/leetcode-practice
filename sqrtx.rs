impl Solution {
	pub fn my_sqrt(number: i32) -> i32 {
		let mut left : i32 = 0;
		let mut right : i32 = number;

		loop {
			if left > right {
				return right;
			}

			let middle : i32 = (left + right) / 2;
			let value : i64 = middle as i64 * middle as i64;

			if value == number as i64 {
				return middle;
			} else if value < number as i64 {
				left = middle + 1;
			} else {
				right = middle - 1;
			}
		}
	}
}
