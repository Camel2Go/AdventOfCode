use std::collections::HashMap;
use std::mem::swap;
use std::time::Instant;
use std::fs;

fn main() {

	let data = fs::read_to_string("data/14").expect("Error while reading data");

	let mut time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());

	time = Instant::now();

	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}


fn run(data: &String, part2: bool) -> usize {
	
	let mut cave: HashMap<(usize, usize), bool> = HashMap::new();

	for rock in data.split("\n") {
		let mut lines = rock.split(" -> ").map(|points|{
			let (x, y) = points.split_once(",").unwrap();
			return (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
		});
		let mut start = lines.next().unwrap();
		while let Some(end) = lines.next() {
			for x in if start.0 < end.0 {start.0..=end.0} else {end.0..=start.0} {
				for y in if start.1 < end.1 {start.1..=end.1} else {end.1..=start.1} {
					cave.insert((x, y), false);
				}
			}
			start = end;
		}	
	}

	let void = cave.keys().map(|&coords| coords.1).max().unwrap();
	let mut rest = false;
	
	while !rest {
		let (mut x, mut y) = (500, 0);
		loop {
			if cave.contains_key(&(x, y + 1)) {

				if !cave.contains_key(&(x - 1, y + 1)) {
					(x, y) = (x - 1, y + 1);
				} else if !cave.contains_key(&(x + 1, y + 1)) {
					(x, y) = (x + 1, y + 1);
				} else {
					cave.insert((x, y), true);
					if part2 && (x, y) == (500, 0) {rest = true;}
					break;
				}
			} else {
				y += 1;
			}

			if part2 {
				if y == void + 1 {
					cave.insert((x, y), true);
					break;
				}
			} else if y == void{
				rest = true;
				break;
			}
		}
	}

	// for y in 0..170 {
	// 	for x in 300..700 {
	// 		let s = if let Some(e) = cave.get(&(x, y)) {
	// 			if *e {"o"} else {"#"}
	// 		} else {"."};
	// 		print!("{}", s);
	// 	}
	// 	print!("\n");
	// }

	return cave.values().filter(|&e| *e).count();
}
