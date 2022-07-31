use std::{env, fs};

fn solve(filename: String) {
	let f = fs::read_to_string(filename).unwrap();
	let lines = f.lines().map(|line| {
		line
			.split('#').next().unwrap()
			.split(char::is_whitespace)
			.filter(|s| s.len() > 0)
			.map(|s| s.parse::<u64>().unwrap())
			.collect::<Vec<u64>>()
	}).filter(|line| line.len() > 0);

	let size = lines.next().unwrap()[0].try_into().unwrap();

	assert!(lines.count() == size);
	for line in lines {
		assert!(line.len() == size);
		for n in line {
			print!("-> [{}]", n);
		}
		println!("");
	}
}

fn main() {
	for arg in env::args().skip(1) {
		solve(arg);
	}
}
