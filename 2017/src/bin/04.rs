use std::time::Instant;
use std::fs;
use itertools::Itertools;

fn main() {

	let data = fs::read_to_string("data/04").expect("Error reading data");

	let now = Instant::now();
	
	println!("{}", run(&data, true));
	println!("{}", run(&data, false));
	println!("======= {}s ========", now.elapsed().as_secs_f32());
}

fn run(data: &str, part1: bool) -> u32 {

	let data_iter = data.split('\n');
	
	if part1 {
		return data_iter.filter(|passphrase| passphrase.split(" ").duplicates().next().is_none()).count() as u32;
	} else {
		return data_iter.filter(|passphrase| passphrase.split(" ").map(|password| password.chars().sorted().collect::<String>()).duplicates().next().is_none()).count() as u32;
	}
}