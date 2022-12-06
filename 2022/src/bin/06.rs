use std::collections::HashSet;
use std::time::Instant;
use std::fs;
use std::hash::Hash;
// use cute::c;

fn main() {

	let data = fs::read_to_string("data/06").expect("Error while reading data");

	let time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());


	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}

fn run(data: &String, part2: bool) -> usize {
	
	let data = data.bytes().collect::<Vec<_>>();
	let size = if part2 {14} else {4};
	for index in 0..data.len() {
		if !(index..index + size).any(|i| data[i + 1..index + size].contains(&data[i])) {
			return index + size;
		}
	}

	return 0 ;
}