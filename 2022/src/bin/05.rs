
use std::time::Instant;
use std::fs;
use cute::c;

fn main() {

	let data = fs::read_to_string("data/05").expect("Error while reading data");

	let time = Instant::now();

	println!("{}", run(&data, false));
	println!("{}", run(&data, true));

	println!("======= {}s ========", time.elapsed().as_secs_f32());
}

fn run(data: &String, part2: bool) -> String {
	let mut data = data.split("\n\n");
	let mut crates = data.next().unwrap().rsplit("\n");
	let steps = data.next().unwrap().split("\n").map(|step| step.split(" ").collect::<Vec<_>>());

	let num_stacks = (crates.next().unwrap().len() + 1) / 4;
	let mut stacks = vec![Vec::new(); num_stacks];
	
	for level in crates.map(|x| x.chars().collect::<Vec<_>>()) {
		for i in 0..num_stacks {
			if level[1 + i * 4].is_ascii_uppercase() {
				stacks[i].push(level[1 + i * 4]);
			}
		}
	}

	for step in steps {
		let count = step[1].parse::<usize>().unwrap();
		let from = step[3].parse::<usize>().unwrap() - 1;
		let to = step[5].parse::<usize>().unwrap() - 1;
		if part2 {
			let start = stacks[from].len() - count;
			let elem = stacks[from].drain( start..).collect::<Vec<_>>();
			stacks[to].extend(elem);
		} else {
			for _ in 0..count {
				let elem = stacks[from].pop().unwrap();
				stacks[to].push(elem);
			}
		}
	}
	

	return c![*stack.last().unwrap(), for stack in stacks].iter().collect::<String>() ;
}