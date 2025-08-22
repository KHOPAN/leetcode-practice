impl Solution {
	pub fn num_submat(matrix: Vec<Vec<i32>>) -> i32 {
		let mut total : i32 = 0;
		let width : usize = matrix[0].len();
		let height : usize = matrix.len();
		let mut row : Vec<Vec<i32>> = vec![vec![0; width]; height];

		for y in 0..height {
			for x in 0..width {
				row[y][x] = if x == 0 { matrix[y][x] } else if matrix[y][x] == 0 { 0 } else { row[y][x - 1] + 1 };
				let mut current : i32 = row[y][x];

				for i in (0..=y).rev() {
					current = current.min(row[i][x]);

					if current == 0 {
						break;
					}

					total += current;
				}
			}
		}

		total
	}
}
