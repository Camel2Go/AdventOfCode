use std::time::Instant;
use std::iter::zip;
use std::fs;

fn main() {

	let data = fs::read_to_string("data/01").expect("Error while reading data");

	let now = Instant::now();
	
	println!("{}", run(&data, true));
	println!("{}", run(&data, false));
	println!("======= {}s ========", now.elapsed().as_secs_f32());
}

fn run(data: &String, part1: bool) -> u32 {

	let mut comp: Vec<char> = data.chars().collect();
	
	if part1 {
		comp.rotate_right(1);
	} else {
		let length = comp.len() / 2;
		comp.rotate_right(length);
	}
	return zip(data.chars(), comp.into_iter()).map(|(current, next)| if current == next {current.to_digit(10).unwrap()} else {0}).sum();
}