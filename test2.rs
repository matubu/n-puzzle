use std::collections::HashMap;

fn build_spiral(n: usize) {
	let mut step_size = n;
	let step = 1;
	let x: usize = 0;
	let y: usize = 0;

	while true {
		match step_size {
			1 => {
				println!("{},{} => {}", x, y, 0);
				break ;
			},
			2 => {
				println!("{},{} => {}", x, y, step);
				println!("{},{} => {}", x + 1, y, step);
				println!("{},{} => {}", x + 1, y + 1, step);
				println!("{},{} => {}", x, y + 1, 0);
			},
			n => {
				step_size -= 2;
			}
		}
	}
}

fn main() {
	build_spiral(5);
}