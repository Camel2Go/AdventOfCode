use std::time::Instant;

fn main() {

	let data = 289326;

	let now = Instant::now();
	
	println!("{}", run(data, true));
	println!("{}", run(data, false));
	println!("======= {}s ========", now.elapsed().as_secs_f32());
}

fn run(data: u32, part1: bool) -> u32 {

	if part1 {
		let mut counter: u32 = 1;
		let mut x: u32 = 0;

		while counter < data {
			x += 1;
			counter += 2 * x;
		}

		let edge = x + (x % 2);

		let value: i32 = edge as i32 / 2 - (counter as i32 - data as i32) % x as i32;

		return edge - (edge / 2 - value.abs() as u32) - ((x % 2 == 1) & (counter - data >= x)) as u32;
	}

	let size = 20;
	let mut grid: Vec<Vec<u32>> = vec![vec![0; size]; size];
	let mut x = size / 2;
	let mut y = size / 2;
	let mut left = 1;
	let mut xleft = left;
	let mut yleft = left;
	let mut xdir: i16 = 1;
	let mut ydir: i16 = 0;
	let mut tmp: i16;
	
	grid[x][y] = 1;

	while grid[x][y] <= data {
		x = (x as i16 + xdir) as usize;
		y = (y as i16 + ydir) as usize;
		
		
		if xleft > 0 {
			xleft -= 1;
			if xleft == 0 {
				tmp = xdir;
				xdir = -ydir;
				ydir = tmp;
			}
		} else if yleft > 0 {
			yleft -= 1;
			if yleft == 0 {
				tmp = xdir;
				xdir = -ydir;
				ydir = tmp;
			}
		} 
		if (xleft == 0) & (yleft == 0) {
			left += 1;
			xleft = left;
			yleft = left;
		}

		grid[x][y] += grid[x - 1][y - 1];
		grid[x][y] += grid[x + 0][y - 1];
		grid[x][y] += grid[x + 1][y - 1];
		grid[x][y] += grid[x - 1][y + 0];
		grid[x][y] += grid[x + 1][y + 0];
		grid[x][y] += grid[x - 1][y + 1];
		grid[x][y] += grid[x + 0][y + 1];
		grid[x][y] += grid[x + 1][y + 1];
	}
	

	// for x in 0..grid.len() {
	// 	for y in 0..grid[x].len() {
	// 		if grid[x][y] != 0 {
	// 			print!("{} ", grid[x][y])
	// 		}
	// 	}
	// 	println!();
	// }

	return grid[x][y];
}