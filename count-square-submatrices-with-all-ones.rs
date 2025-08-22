impl Solution {
	fn check_square(matrix: &Vec<Vec<i32>>, x: usize, y: usize, size: usize) -> bool {
		for height in 0..size {
			for width in 0..size {
				if matrix[y + height][x + width] != 1 {
					return false;
				}
			}
		}

		true
	}

	pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
		let width : usize = matrix[0].len();
		let height : usize = matrix.len();
		let cube : usize = if width < height { width } else { height };
		let mut total : i32 = 0;

		for size in 0..cube {
			let mut count : i32 = 0;

			for y in 0..(height - size) {
				for x in 0..(width - size) {
					if Self::check_square(&matrix, x, y, size + 1) {
						count += 1;
					}
				}
			}

			if count < 1 {
				break;
			}

			total += count;
		}

		total
	}
}
