#![feature(is_some_and)]
use std::collections::HashMap;
use std::time::Instant;
use std::fs;

use itertools::Itertools;
// use regex::Regex;
// use itertools::Itertools;
// use cute::c;

fn main() {

	let data = fs::read_to_string("data/12").expect("Error while reading data");

	let mut time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());

	time = Instant::now();

	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}

fn run(data: &String, part2: bool) -> usize {
	
	let grid: Vec<Vec<usize>> = data.replace("S", "a").replace("E", "z").split("\n").map(|l| l.chars().map(|s| s as usize - 'a' as usize).collect()).collect();
	let mut dict: HashMap<(usize, usize), usize> = HashMap::new();
	let mut queue: Vec<((usize, usize), usize)> = Vec::new();

	let s = if let Some(x) = data.replace("\n", "").find("S") {
		(x / grid[0].len(), x % grid[0].len())
	} else {panic!()};
	
	queue.push((s, 0));

	if part2 {
		for y in 0..grid.len() {
			for x in 0..grid[0].len() {
				if grid[y][x] == 0 {
					queue.push(((y, x), 0));
				}
			}
		}
	}

	let e = if let Some(x) = data.replace("\n", "").find("E") {
		(x / grid[0].len(), x % grid[0].len())
	} else {panic!()};


	while !dict.contains_key(&e) {
		
		let ((y, x), dist) = queue.remove(0);
		
		for (ny, nx) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
			if ny < grid.len() && nx < grid[0].len() && grid[ny][nx] <= grid[y][x] + 1 {
				if dict.contains_key(&(ny, nx)) {
					dict.entry((ny, nx)).and_modify(|h| if *h > dist + 1 {println!("abc"); *h = dist + 1});
				} else {
					dict.insert((ny, nx), dist + 1);
					queue.push(((ny, nx), dist + 1));
				}
			}
		}
	}

	return dict[&e];
}
