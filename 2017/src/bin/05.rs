use std::time::Instant;
use std::fs;

fn main() {

	let data = fs::read_to_string("data/05").expect("Error reading data");

	let now = Instant::now();

	println!("{}", run(&data, true));
	println!("{}", run(&data, false));

	println!("======= {}s ========", now.elapsed().as_secs_f32());
}

fn run(data: &String, part1: bool) -> u32 {

	let mut data: Vec<i16> = data.split('\n').map(|number| number.parse::<i16>().unwrap()).collect();
	
	let mut index = 0;
	let mut steps = 0;

	while index < data.len().try_into().unwrap() {
		steps += 1;
		let tmp = data[index];
		data[index] += if !part1 & (data[index] >= 3) {-1} else {1};
		index = (index as i16 + tmp) as usize;
	}

	return steps;
}