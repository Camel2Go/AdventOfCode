use std::cmp::Ordering;
use std::iter::zip;
use std::time::Instant;
use std::fs;
use itertools::Itertools;
use json::JsonValue;
// use regex::Regex;
// use itertools::Itertools;
// use cute::c;

fn main() {

	let data = fs::read_to_string("data/13").expect("Error while reading data");

	let mut time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());

	time = Instant::now();

	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}

fn compare(left: &JsonValue, right: &JsonValue, order: &mut isize) {

	if left.is_array() && right.is_array() {
	
		for (l, r) in zip(left.members(), right.members()) {
			
			compare(l, r, order);
			if *order != 0 {return;}
		}
	
		if left.members().count() < right.members().count() {*order = -1; return;}
		if left.members().count() > right.members().count() {*order = 1; return;}
	
	} else if left.is_array() && right.is_number() {
		
		if left.members().count() < 1 {*order = -1; return;}
		
		compare(left.members().next().unwrap(), right, order);
		if *order != 0 {return;}

		if left.members().count() > 1 {*order = 1; return;}
	
	} else if left.is_number() && right.is_array() {
		
		if right.members().count() < 1 {*order = 1; return;}

		compare(left, right.members().next().unwrap(), order);
		if *order != 0 {return;}

		if right.members().count() > 1 {*order = -1; return;}

	} else if left.is_number() && right.is_number() {
		if left.as_usize() < right.as_usize() {*order = -1; return;}
		if left.as_usize() > right.as_usize() {*order = 1; return;}
	}
	return;
}

fn cmp_packets(left: &str, right: &str) -> Ordering {
	let mut order = 0;
	let left = json::parse(left).unwrap();
	let right = json::parse(right).unwrap();
	
	compare(&left, &right, &mut order);
	return match order {
		-1 => Ordering::Less,
		0 => Ordering::Equal,
		1 => Ordering::Greater,
		_ => panic!()
	};
}

fn run(data: &String, part2: bool) -> usize {
	
	return if part2 {
		let mut packets: Vec<&str> = data.split("\n").filter(|p| *p != "").collect();
		packets.push("[[2]]");
		packets.push("[[6]]");
		packets.sort_unstable_by(|p1, p2| cmp_packets(p1, p2));
		(packets.iter().position(|p| *p == "[[2]]").unwrap() + 1) * (packets.iter().position(|p| *p == "[[6]]").unwrap() + 1)
	} else {
		data.split("\n\n").map(|pair| {
			let (left, right) = pair.split("\n").next_tuple().unwrap();
			return cmp_packets(left, right);
		}).enumerate().filter(|(_, val)| *val != Ordering::Greater).map(|(i, _)| i + 1).sum()
	}
	

}
