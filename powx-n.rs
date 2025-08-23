impl Solution {
	pub fn my_pow(mut number: f64, mut power: i32) -> f64 {
		let negative : bool = power.is_negative();
		let mut value : f64 = 1.0;

		while power != 0 {
			if power % 2 != 0 {
				value *= number;
			}

			number *= number;
			power /= 2;
		}

		if negative {
			1.0f64 / value
		} else {
			value
		}
	}
}
