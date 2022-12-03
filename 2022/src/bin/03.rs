#![feature(iter_next_chunk)]

use std::collections::HashSet;
use std::time::Instant;
use std::fs;

fn main() {

	let data = fs::read_to_string("data/03").expect("Error while reading data");

	let time = Instant::now();

	println!("{}", run(&data, false));
	println!("{}", run(&data, true));

	println!("======= {}s ========", time.elapsed().as_secs_f32());
}

fn run(data: &String, part2: bool) -> usize {

	let mut rucksacks = data.split("\n");
	let mut priority = 0;

	if part2 {
		while rucksacks.clone().peekable().peek().is_some() {
			let [first, second, third] = rucksacks.next_chunk().unwrap().map(|x| x.chars().collect::<HashSet<char>>());

			if let Some(common) = (&(&first & &second) & &third).iter().next() {
				priority += if common.is_lowercase() {*common as usize - 'a' as usize} else {*common as usize - 'A' as usize + 26} + 1
			}
		}
	} else {
		for rucksack in rucksacks {
	
			let first = rucksack.get(0..rucksack.len() / 2).unwrap().chars().collect::<HashSet<char>>();
			let second = rucksack.get(rucksack.len() / 2..rucksack.len()).unwrap().chars().collect::<HashSet<char>>();
			
			if let Some(common) = first.intersection(&second).next() {
				priority += if common.is_lowercase() {*common as usize - 'a' as usize} else {*common as usize - 'A' as usize + 26} + 1
			}
		}
	}

	return priority;
}