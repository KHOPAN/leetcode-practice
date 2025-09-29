impl Solution {
	pub fn generate(rows: i32) -> Vec<Vec<i32>> {
		let mut result : Vec<Vec<i32>> = Vec::new();

		for x in 1..=rows {
			if x == 1 {
				result.push(vec![1]);
				continue;
			}

			let last : &Vec<i32> = result.last().unwrap();
			let mut array : Vec<i32> = Vec::new();

			for y in 0..x {
				array.push(if y == 0 { 0 } else { last[(y - 1) as usize] } + if y == x - 1 { 0 } else { last[y as usize] });
			}

			result.push(array);
		}

		result
	}
}
