impl Solution {
	pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
		let width : usize = grid[0].len();
		let height : usize = grid.len();
		let mut minimum_x : usize = width - 1;
		let mut minimum_y : usize = height - 1;
		let mut maximum_x : usize = 0;
		let mut maximum_y : usize = 0;

		for y in 0..height {
			for x in 0..width {
				if grid[y][x] == 1 {
					minimum_x = minimum_x.min(x);
					minimum_y = minimum_y.min(y);
					maximum_x = maximum_x.max(x);
					maximum_y = maximum_y.max(y);
				}
			}
		}

		((maximum_x - minimum_x + 1) * (maximum_y - minimum_y + 1)) as i32
	}
}
