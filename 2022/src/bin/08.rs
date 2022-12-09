#![feature(let_chains)]
use std::time::Instant;
use std::fs;

use cute::c;

// use cute::c;

fn main() {

	let data = fs::read_to_string("data/08").expect("Error while reading data");

	let mut time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());

	time = Instant::now();

	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}

fn run(data: &String, part2: bool) -> usize {
	
	let trees: Vec<Vec<usize>> = data.split("\n").map(|r| r.chars().map(|e| e as usize - '0' as usize).collect()).collect();
	let size = trees.len();
	let mut grid: Vec<Vec<usize>>;

	if part2 {
		
		grid = vec![vec![1; trees[0].len()]; trees.len()];
		

		for i in 1..size - 1 {
			for j in 1..size - 1 {
				let mut up = 1;
				let mut down = 1;
				let mut left = 1;
				let mut right = 1;
				
				while trees[i][j] > trees[i - up][j] && i - up > 0 {up += 1;}
				while trees[i][j] > trees[i + down][j] && i + down < size - 1 {down += 1;}
				while trees[i][j] > trees[i][j - left] && j - left > 0 {left += 1;}
				while trees[i][j] > trees[i][j + right] && j + right < size - 1 {right += 1;}

				grid[i][j] = up * down * left * right;
			}

		}
		for i in 0..size {
			grid[i][0] = 0; 
			grid[i][size - 1] = 0; 
			grid[0][i] = 0; 
			grid[size - 1][i] = 0; 
		}

	} else {

		grid = vec![vec![0; trees[0].len()]; trees.len()];

		for i in 0..size {
			grid[i][0] = 1; 
			grid[i][size - 1] = 1; 
			grid[0][i] = 1; 
			grid[size - 1][i] = 1; 
		}
		
		for i in 1..size - 1 {
			let mut upper_max = trees[0][i];
			let mut lower_max = trees[size - 1][i];
			let mut lefter_max = trees[i][0];
			let mut righter_max = trees[i][size - 1];
			for up in 1..size - 1 {
				if trees[up][i] > upper_max {
					grid[up][i] = 1;
					upper_max = trees[up][i];
				}
			}
			for down in (1..size - 1).rev() {
				// if trees[down][i] == upper_max {break;}
				if trees[down][i] > lower_max {
					grid[down][i] = 1;
					lower_max = trees[down][i];
				}
			}
			for left in 1..size - 1 {
				if trees[i][left] > lefter_max {
					grid[i][left] = 1;
					lefter_max = trees[i][left];
				}
			}
			for right in (1..size - 1).rev() {
				// if trees[i][right] == lefter_max {break;}
				if trees[i][right] > righter_max {
					grid[i][right] = 1;
					righter_max = trees[i][right];
				}
			}
		}
	}

		
	// for row in 0..size {
	// 	println!("{:}", grid[row].iter().map(|e| (*e as usize).to_string()).collect::<String>())
	// }
	
	return if part2{
		grid.iter().map(|r| *r.iter().max().unwrap()).max().unwrap()
	} else {
		grid.iter().map(|r| r.iter().sum::<usize>()).sum()
	}
}