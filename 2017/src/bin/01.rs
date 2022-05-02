use std::fs;
use std::iter;

fn main() {

    let data = fs::read_to_string("data/01").expect("Error while reading data");
    println!("{}\n", run(data));
}

fn run(data: String) -> u32 {

    let mut sum: u32 = 0;
    let mut data = data.chars();
    let first = data.next();
    while let second = data.next() {
        if second == None {
            if first == data.nth(0) {
                sum += first.unwrap().to_digit(10).unwrap();
            }
            break;
        }
        if first == second {
            sum += first.unwrap().to_digit(10).unwrap();
        }
        let first = second;
    }
    // let sum = data.chars().zip(iter::once(data.get(0)).chain(data.chars().skip(1))).filter(|(current, next)| current == next).sum();
    return sum;
}