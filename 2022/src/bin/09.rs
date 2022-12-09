#![feature(let_chains)]
use std::collections::HashMap;
use std::time::Instant;
use std::fs;

use cute::c;
use itertools::Itertools;

// use cute::c;

fn main() {

	let data = fs::read_to_string("data/09").expect("Error while reading data");

	let mut time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());

	time = Instant::now();

	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}

fn run(data: &String, part2: bool) -> usize {
	
	let mut rope = vec![(0, 0); if part2{10} else {2}];
	let mut visited = HashMap::<(isize, isize), bool>::new();
	let dist = |one: (isize, isize), two: (isize, isize)| ((one.0 - two.0).abs()).max((one.1 - two.1).abs());

	for step in data.split('\n') {
		let (direction, steps) = step.split(' ').next_tuple().unwrap();
		for _ in 0..steps.parse().unwrap() {
			match direction {
				"R" => rope[0].0 += 1,
				"L" => rope[0].0 -= 1,
				"U" => rope[0].1 += 1,
				"D" => rope[0].1 -= 1,
				_ => panic!(),
			}
			for knot in 1..rope.len() {
				if dist(rope[knot - 1], rope[knot]) > 1 {
					rope[knot].0 += (rope[knot - 1].0 - rope[knot].0).signum();
					rope[knot].1 += (rope[knot - 1].1 - rope[knot].1).signum();
				}
			}
			visited.insert(*rope.last().unwrap(), true);
		}
	}
	return visited.len();
}