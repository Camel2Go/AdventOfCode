use std::time::Instant;
use std::fs;

fn main() {

	let data = fs::read_to_string("data/01").expect("Error while reading data");

	let now = Instant::now();
	
	println!("{}", run(&data, true));
	println!("{}", run(&data, false));
	println!("======= {}s ========", now.elapsed().as_secs_f32());
}

fn run(data: &String, part1: bool) -> usize {

	// let mut elves: Vec<usize> = Vec::new();
	// let mut calories = 0;
	// for food in data.split("\n").collect::<Vec<&str>>() {
	// 	if food == "" {
	// 		elves.push(calories);
	// 		calories = 0;
	// 	} else {
	// 		calories += food.parse::<usize>().unwrap();
	// 	}
	// }

	let mut elves: Vec<usize> = data.split("\n\n").into_iter().map(|elve| elve.split("\n").into_iter().map(|x| x.parse::<usize>().unwrap()).sum()).collect();

	elves.sort();

	return if part1 {elves.last().copied().unwrap_or_default()} else {elves.iter().rev().take(3).sum()};
}