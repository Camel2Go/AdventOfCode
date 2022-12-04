use std::time::Instant;
use std::fs;

fn main() {

	let data = fs::read_to_string("data/04").expect("Error while reading data");

	let time = Instant::now();

	println!("{}", run(&data, false));
	println!("{}", run(&data, true));

	println!("======= {}s ========", time.elapsed().as_secs_f32());
}

fn run(data: &String, part2: bool) -> usize {

	let mut count = 0;
	for pair in data.split("\n").map(|x| x.replace(',', "-").split('-').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()) {
		
		count += if part2 {
			pair[0] <= pair[3] && pair[2] <= pair[1]
		} else {
			pair[0] <= pair[2] && pair[1] >= pair[3] || pair[0] >= pair[2] && pair[1] <= pair[3]
		} as usize;
	}

	return count;
}