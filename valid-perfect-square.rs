impl Solution {
	pub fn is_perfect_square(number: i32) -> bool {
		let mut left : i32 = 0;
		let mut right : i32 = number;

		loop {
			if left > right {
				return false;
			}

			let middle : i32 = (left + right) / 2;
			let value : i64 = middle as i64 * middle as i64;

			if value == number as i64 {
				return true;
			} else if value < number as i64 {
				left = middle + 1;
			} else {
				right = middle - 1;
			}
		}
	}
}
