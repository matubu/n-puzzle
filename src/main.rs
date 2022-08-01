use std::{env, fs};

fn parse(filename: String) -> Vec<Vec<u64>> {
	let f = fs::read_to_string(filename).unwrap();
	let lines = &mut f.lines().map(|line| {
		line
			.split('#').next().unwrap()
			.split(char::is_whitespace)
			.filter(|s| s.len() > 0)
			.map(|s| s.parse::<u64>().unwrap())
			.collect::<Vec<u64>>()
	}).filter(|line| line.len() > 0);

	let sizeline = lines.next().unwrap();
	assert!(sizeline.len() == 1);
	let size: u64 = sizeline[0];
	let u_size: usize = size.try_into().unwrap();

	let vec: Vec<Vec<u64>> = lines.collect();
	assert!(vec.len() == u_size);

	for line in &vec {
		assert!(line.len() == u_size);
		for x in line {
			assert!(*x < (size * size));
		}
	}

	vec
}

fn main() {
	for arg in env::args().skip(1) {
		println!("{:?}", parse(arg));
	}
}
