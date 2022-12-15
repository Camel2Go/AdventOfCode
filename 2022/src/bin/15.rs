use std::collections::HashSet;
use std::time::Instant;
use std::fs;

use text_io::scan;


fn main() {

	let data = fs::read_to_string("data/15").expect("Error while reading data");

	let mut time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());

	time = Instant::now();

	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}


fn run(data: &String, part2: bool) -> usize {
	
	let dist = |sx: isize, sy: isize, bx: isize, by: isize| (sx - bx).abs() + (sy - by).abs();

	let mut sensors = Vec::new();
	let mut beacons = Vec::new();

	for sensor in data.split("\n") {
		
		let (sx, sy, bx, by): (isize, isize, isize, isize);
		scan!(sensor.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by);
		sensors.push((sx, sy, dist(sx, sy, bx, by)));
		beacons.push((bx, by));
	}

	if part2 {
		let check = |x: isize, y: isize| x >= 0 && x <= 4000000 && y >= 0 && y <= 4000000 && sensors.iter().all(|s| dist(s.0, s.1, x, y) > s.2);

		for &s in sensors.iter() {
			// println!("processing next scanner...");
			for i in 0..=s.2 {
				let (x, y) = (s.0 - s.2 + i - 1, s.1 - i);
				if check(x, y) {return (x * 4000000 + y) as usize;}
				let (x, y) = (s.0 - s.2 + i - 1, s.1 + i);
				if check(x, y) {return (x * 4000000 + y) as usize;}
				let (x, y) = (s.0 + s.2 - i + 1, s.1 - i);
				if check(x, y) {return (x * 4000000 + y) as usize;}
				let (x, y) = (s.0 + s.2 - i + 1, s.1 + i);
				if check(x, y) {return (x * 4000000 + y) as usize;}
			}
			let (x, y) = (s.0, s.1 - s.2 - 1);
			if check(x, y) {return (x * 4000000 + y) as usize;}
			let (x, y) = (s.0, s.1 + s.2 + 1);
			if check(x, y) {return (x * 4000000 + y) as usize;}
		}

		println!("no exact position found!");
		return 0;
	
	} else {
		let mut row = HashSet::new();
		for &s in sensors.iter() {
			let dist = (s.1 - 2000000).abs();
				if dist <= s.2 {
					for i in s.0 - (s.2 - dist)..=s.0 + (s.2 - dist) {
						row.insert(i);
					}
				}
			}
		return row.difference(&beacons.iter().filter_map(|&b| if b.1 == 2000000 {Some(b.0)} else {None}).collect()).count()
	}

}
