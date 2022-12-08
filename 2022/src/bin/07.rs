#![feature(entry_insert)]
use std::collections::HashMap;
use std::time::Instant;
use std::{fs, iter};

use itertools::Itertools;
// use cute::c;

fn main() {

	let data = fs::read_to_string("data/07").expect("Error while reading data");

	let time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());


	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}

// fn calc_sum(dir: Vec<String>, dirs: &HashMap<Vec<String>, Vec<String>>, files: &HashMap<String, usize>) -> usize {
// 	let mut ret = 0;
// 	for entry in &dirs[&dir] {
// 		ret += if let Some(size) = files.get(entry) {
// 			*size
// 		} else {
// 			let mut new_dir = dir.clone();
// 			new_dir.push(entry.clone());
// 			calc_sum(new_dir, dirs, files)
// 		};
// 	}
// 	dbg!(dir, ret);
// 	return ret;
// }

fn absolut(current: Vec<String>, name: String) -> String {
	return current.into_iter().chain(iter::once(name)).collect::<Vec<String>>().join("/")
}


fn run(data: &String, part2: bool) -> usize {
	
	let mut dirs = HashMap::new();
	let mut sizes: HashMap<String, usize> = HashMap::new();
	let mut current = vec![String::from("")];

	for mut line in data.split("\n").map(|l| l.chars()) {
		let first = line.take_while_ref(|e| *e != ' ').collect::<String>();
		if first.contains('$') {
			if line.by_ref().take(4).eq(" cd ".chars()) {
				match line.as_str() {
					"/" => current.truncate(1),
					".." => drop(current.pop()),
					dir => current.push(dir.to_string()),
				};
			}
		} else {
			let name = line.skip(1).collect::<String>();
			dirs.entry(current.clone()).or_insert(Vec::new()).push(name.clone());
			first.parse::<usize>().and_then(|len| Ok(sizes.insert(absolut(current.clone(), name), len)));
		}
	}
	for dir in dirs.keys().sorted_by_key(|v| v.len()).rev() {
		let idk = dirs[dir].iter().map(|f| sizes[&absolut(dir.clone(), f.clone())]).sum();
		sizes.entry(dir.join("/")).insert_entry(idk);
	}

	return if part2 {
		let needed = 30000000 + sizes[""] - 70000000;
		dirs.keys().map(|d| sizes[&d.join("/")]).filter(|s| *s >= needed).min().unwrap()
	} else {
		dirs.keys().map(|d| sizes[&d.join("/")]).filter(|s| *s <= 100000).sum()
	}
}