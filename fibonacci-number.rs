impl Solution {
	pub fn fib(number: i32) -> i32 {
		if number == 0 || number == 1 {
			return number;
		}

		let mut result : i32 = 1;
		let mut adder : i32 = 1;

		for _ in 2..number {
			let previous : i32 = result;
			result += adder;
			adder = previous;
		}

		result
	}
}
