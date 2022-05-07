use std::time::Instant;
use std::fs;
use itertools::Itertools;

fn main() {

	let data = fs::read_to_string("data/02").expect("Error reading data");

	let now = Instant::now();
	
	println!("{}", run(&data, true));
	println!("{}", run(&data, false));
	println!("======= {}s ========", now.elapsed().as_secs_f32());
}

fn run(data: &String, part1: bool) -> u32 {

	let data: Vec<Vec<u32>> = data.split('\n').map(|line| line.split('\t').map(|number| number.parse::<u32>().unwrap()).collect()).collect();
	
	if part1 {
		return data.iter().map(|line| line.iter().max().unwrap() - line.iter().min().unwrap()).sum();

	} else {
		return data.iter().map(|line| line.iter().permutations(2).map(|tuple| if tuple[0] % tuple[1] == 0 {tuple[0] / tuple[1]} else {0}).sum::<u32>()).sum();
	}

}