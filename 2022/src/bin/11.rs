use std::borrow::BorrowMut;
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

	println!("[2\u{fe0f}\u{20e3}\u{fe0f} ] {}", run(&data, true));
	println!("[\u{23f1}\u{fe0f} ] {}s", time.elapsed().as_secs_f32());
}
#[derive(Clone, Default, Debug)]
struct Monkey {
	items: Vec<usize>,
	operation: char,
	operand: usize,
	test: usize,
	monk_true: usize,
	monk_false: usize,
	inspected: usize,
}

fn run(data: &String, part2: bool) -> usize {
	
	let mut monkeys = Vec::new();
	for mut attrib in data.split("\n\n").map(|m| m.split("\n").skip(1)) {
		let items = attrib.next().unwrap().chars().skip(18).collect::<String>().split(", ").map(|i| i.parse::<usize>().unwrap()).collect();
		let tmp = attrib.next().unwrap().chars().skip(23).collect::<Vec<char>>();
		let operation = tmp[0];
		let operand = tmp[2..].iter().collect::<String>().parse::<usize>().unwrap_or_default();
		let test = attrib.next().unwrap().chars().skip(21).collect::<String>().parse::<usize>().unwrap();
		let monk_true = attrib.next().unwrap().chars().skip(29).collect::<String>().parse::<usize>().unwrap();
		let monk_false = attrib.next().unwrap().chars().skip(30).collect::<String>().parse::<usize>().unwrap();
		let inspected = 0;

		monkeys.push(Monkey {items, operation, operand, test, monk_true, monk_false, inspected})
	}
	let modul: usize = monkeys.iter().map(|m| m.test).product();


	for _ in 0..if part2 {10000} else {20} {
		for monkey in 0..monkeys.len() {
			let monkey = monkeys[monkey].borrow_mut();
			let mut throw = Vec::new();
			for mut item in monkey.items.clone() {
				item = match (monkey.operation, monkey.operand.clone()) {
					('+', 0) => 2 * item,
					('*', 0) => item * item,
					('+', x) => item + x,
					('*', x) => item * x,
					(_, _) => panic!(),
				} % modul;
				if !part2 {item /= 3};
				let other = if item % monkey.test == 0 {
					monkey.monk_true
				} else {
					monkey.monk_false
				};
				throw.push((other, item));
				monkey.inspected += 1;
			}
			monkey.items.clear();
			for (other, item) in throw.iter() {
				monkeys[*other].items.push(*item)
			};
		}
	}
	monkeys.sort_by_key(|m| m.inspected);

	return monkeys.iter().rev().take(2).map(|m| m.inspected).product();
}
