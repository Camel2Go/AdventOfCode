use std::env::VarError;
use std::num::ParseIntError;
use std::time::Instant;
use std::fs;
// use regex::Regex;
// use itertools::Itertools;
// use cute::c;

fn main() {

	let data = fs::read_to_string("data/11").expect("Error while reading data");

	let mut time = Instant::now();

	println!("[1\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, false));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());

	time = Instant::now();
// 
	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}

struct Monkey {
	items: Vec<usize>,
	operation: char,
	operand: Result<usize, ParseIntError>,
	test: usize,
	monk_true: usize,
	monk_false: usize,
}

fn run(data: &String, part2: bool) -> usize {
	
	let mut monkeys = Vec::<Monkey>::new();
	for mut attrib in data.split("\n\n").map(|m| m.split("\n").skip(1)) {
		let items = attrib.next().unwrap().chars().skip(18).collect::<String>().split(", ").map(|i| i.parse::<usize>().unwrap()).collect();
		
		let tmp = attrib.next().unwrap().chars().skip(23).collect::<Vec<char>>();
		let operation = tmp[0];
		let operand = tmp[2..].iter().collect::<String>().parse::<usize>();

		let test = attrib.next().unwrap().chars().skip(21).collect::<String>().parse::<usize>().unwrap();
		let monk_true = attrib.next().unwrap().chars().skip(29).collect::<String>().parse::<usize>().unwrap();
		let monk_false = attrib.next().unwrap().chars().skip(30).collect::<String>().parse::<usize>().unwrap();


		let monkey = Monkey {items: items, operation: operation, operand: operand, test: test, monk_true: monk_true, monk_false: monk_false};
		monkeys.push(monkey)
	}
	for mut monkey in monkeys {
		for mut item in monkey.items {
			item = match (monkey.operation, monkey.operand) {
				('+', Ok(x)) => item + x,
				('*', Ok(x)) => item * x,
				('+', Error) => 2 * item,
				('*', Error) => item * item,
				(_, _) => panic!(),
			};
			item /= 3;
			if (item % monkey.test == 0) {
				monkeys[monkey.monk_true].items.push(item);
			} else {
				monkeys[monkey.monk_false].items.push(item);
			}
			monkey.items.clear();
		}
	}
// };
	return 0;
}
