use std::time::Instant;
use std::fs;
// use itertools::Itertools;
// use cute::c;

fn main() {

	let data = fs::read_to_string("data/10").expect("Error while reading data");

	let mut time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());

// 	time = Instant::now();
// 
// 	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
// 	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}

fn run(data: &String, part2: bool) -> usize {
	
	let mut x = 1;
	let mut cycle = 1;
	let mut strength = 0;
	let mut crt = Vec::new();

	for mut line in data.split('\n').map(|l| l.split(' ')) {
		
		if (cycle - 20) % 40 == 0 {
			strength += cycle * x;
		}
		crt.push(if (0..3).contains(&(x + 1 - ((cycle - 1) % 40) as isize)){'#'} else {'.'} );
		if line.next() == Some("addx") {
			if (cycle - 19) % 40 == 0 {
				strength += (cycle + 1) * x;
			}
			crt.push(if (0..3).contains(&(x + 1 - (cycle % 40) as isize)){'#'} else {'.'} );
			x += line.next().unwrap().parse::<isize>().unwrap();
			cycle += 2;
		} else {
			cycle += 1;
		}
	}
	for i in (40..crt.len()).step_by(40) {
		crt.insert(i + i / 40 - 1, '\n')
	}
	println!("{}", crt.iter().collect::<String>());
	return strength as usize;
}