use std::time::Instant;
use std::fs;

fn main() {

	let data = fs::read_to_string("data/06").expect("Error reading data");

	let now = Instant::now();

	println!("{:?}", run(&data));

	println!("======= {}s ========", now.elapsed().as_secs_f32());
}

fn run(data: &String) -> (u32, u32) {

	let mut data: Vec<u32> = data.split('\t').map(|number| number.parse::<u32>().unwrap()).collect();
	let mut configs = Vec::new();
	let mut blocks;
	let mut cycles = 0;
	let mut index;

	while !configs.contains(&data) {
		configs.push(data.clone());
		cycles += 1;
		blocks = *data.iter().max().unwrap();
		index = data.iter().position(|x| *x == blocks).unwrap();
		data[index] = 0;
		for _ in 0..blocks {
			index = (index + 1) % data.len();
			data[index] += 1;
		}
	}
	
	return (cycles, cycles - configs.iter().position(|state| *state == data).unwrap() as u32);
}