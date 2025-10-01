impl Solution {
	pub fn max_area(height: Vec<i32>) -> i32 {
		let mut left_pointer : usize = 0;
		let mut right_pointer : usize = height.len() - 1;
		let mut maximum_area : i32 = 0;

		while left_pointer < right_pointer {
			maximum_area = std::cmp::max(maximum_area, (right_pointer - left_pointer) as i32 * std::cmp::min(height[left_pointer], height[right_pointer]));

			if height[left_pointer] < height[right_pointer] {
				left_pointer += 1;
			} else {
				right_pointer -= 1;
			}
		}

		maximum_area
	}
}
