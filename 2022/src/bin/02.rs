use std::time::Instant;
use std::fs;

fn main() {

	let data = fs::read_to_string("data/02").expect("Error while reading data");

	let time = Instant::now();

	println!("{}", run(&data, false));
	println!("{}", run(&data, true));

	println!("======= {}s ========", time.elapsed().as_secs_f32());
}

fn run(data: &String, part2: bool) -> usize {

	let mut score = 0;
	for round in data.split("\n") {
		let opponent = round.chars().nth(0).unwrap() as usize - 'A' as usize + 1;
		let mut response = round.chars().nth(2).unwrap() as usize - 'X' as usize + 1;
		if part2 {
			response = match response {
				1 => (opponent + 1) % 3 + 1,
				2 => opponent,
				3 => opponent % 3 + 1,
				_ => panic!(),
			}
		}
		score += if opponent == response {
			3
		} else if response == opponent + 1 || response == (opponent + 1) % 3 {
			6
		} else {
			0
		};

		score += response;
	}

	return score;
}