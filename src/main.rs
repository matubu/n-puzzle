use std::{env, fs};
use std::rc::Rc;
use std::collections::HashMap;

type Puzzle = Rc<Vec<Vec<usize>>>;

#[derive(Clone)]
struct Node {
	previous: Option<Puzzle>,
	depth: usize
}

// TODO avoid clone
// TODO do not recalculate position of the 0 position
// TODO remove unwrap... and create better error handling

fn parse(filename: String) -> Puzzle {
	let f = fs::read_to_string(filename).unwrap();
	let lines = &mut f.lines().map(|line| {
		line
			.split('#').next().unwrap()         // Remove comments
			.split(char::is_whitespace)         // Split on spaces
			.filter(|s| s.len() > 0)            // Remove empty strings
			.map(|s| s.parse::<usize>().unwrap()) // Convert to unsigned ints
			.collect::<Vec<usize>>()              // Convert to vector
	}).filter(|line| line.len() > 0);   // Remove empty lines

	let sizeline = lines.next().unwrap();
	assert!(sizeline.len() == 1);
	let size = sizeline[0];

	let vec: Puzzle = lines.collect();
	assert!(vec.len() == size);

	for line in &vec {
		assert!(line.len() == size);
		for x in line {
			assert!(*x < (size * size));
			// TODO assert duplicate
		}
	}

	vec
}

fn abs_diff(a: usize, b: usize) -> usize {
	if a > b {
		a - b
	} else {
		b - a
	}
}

fn manhattan(target: (usize, usize), pos: (usize, usize)) -> usize {
	abs_diff(target.0, pos.0) + abs_diff(target.1, pos.1)
}

// https://math.stackexchange.com/questions/163080/on-a-two-dimensional-grid-is-there-a-formula-i-can-use-to-spiral-coordinates-in#answer-163101
// TODO make the subject version of this function
// Stackoverflow:
// 16 15 14 13
//  5  4  3 12
//  6  1  2 11
//  7  8  9 10
// Subject:
//  1  2  3  4
// 12 13 14  5
// 11  0 15  6
// 10  9  8  7
// Current:
//  0  1  2  3
//  4  5  6  7
//  8  9 10 11
// 12 13 14 15
fn get_spiral_position(i: usize, n: usize) -> (usize, usize) {
	(i % n, i / n)
}

fn distance(state: &Puzzle) -> usize {
	let mut dist: usize = 0;

	for (y, row) in state.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if *cell == 0 {
				continue ;
			}
			let target = get_spiral_position(*cell, state.len());
			dist += manhattan(target, (x, y));
		}
	}

	dist
}

fn select_best(set: &HashMap<Puzzle, Node>) -> Puzzle {
	set.iter().min_by_key(|(puzzle, state)|
		distance(puzzle) * 4
		+ state.depth
	).unwrap().0
}

// - If puzzle is unsolvable -> inform the user and exit

// - Total number of states ever selected in the "opened" set (complexity in time)
// - Maximum number of states ever represented in memory at the same time
// during the search (complexity in size)
// - Number of moves to solve the puzzle
// - The sequence of states to solve the puzzle

fn find_in_puzzle(puzzle: &Puzzle, searched: usize) -> (usize, usize) {
	for (y, row) in puzzle.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if *cell == searched {
				return (x, y);
			}
		}
	}
	panic!("Not found");
}

fn expand(puzzle: Puzzle) -> Vec<Puzzle> {
	let mut possibles_states = Vec::new();
	let (tx, ty) = find_in_puzzle(puzzle, 0);

	let mut add_state = |ox: isize, oy: isize| {
		if     (ox < 0 && tx <= 0)
			|| (oy < 0 && ty <= 0)
			|| (ox > 0 && tx >= puzzle.len()-1)
			|| (oy > 0 && ty >= puzzle.len()-1) {
			return ;
		}
		let x: usize = (tx as isize + ox) as usize;
		let y: usize = (ty as isize + oy) as usize;

		let mut new_state = puzzle.clone();
		new_state[ty][tx] = new_state[y][x];
		new_state[y][x] = 0;

		possibles_states.push(new_state);
	};

	add_state(-1,  0);
	add_state( 1,  0);
	add_state( 0, -1);
	add_state( 0,  1);

	possibles_states
}

fn print_puzzle(puzzle: &Puzzle, previous: Option<(usize, usize)>) -> Option<(usize, usize)> {
	let mut pos: Option<(usize, usize)> = None;

	for (y, row) in puzzle.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if *cell == 0 {
				print!("\x1B[1;91m");
				pos = Some((x, y));
			}
			match previous {
				Some(prev) => {
					if x == prev.0 && y == prev.1 {
						print!("\x1B[1;92m");
					}
				}
				_ => ()
			}
			print!(" {:3}\x1B[0m", cell);
		}
		println!("");
	}

	pos
}

fn reconstruct(map: &HashMap<Puzzle, Node>, final_state: &Puzzle) {
	let mut state: Option<&Puzzle> = Some(final_state);
	let mut path = Vec::<Puzzle>::new();

	println!("Reconstructing...");
	while state != None {
		path.push(state.unwrap().to_vec());
		state = map.get(state.unwrap()).unwrap().previous;
	}
	let mut previous: Option<(usize, usize)> = None;
	for (i, state) in path.iter().rev().enumerate() {
		println!("step {i}:");
		previous = print_puzzle(&state, previous);
	}
	println!("Number of moves                       : {}", path.len());
}

fn solve(puzzle: Puzzle) {
	println!("Solving...");

	let mut opened: HashMap<Puzzle, Node> = HashMap::new();
	let mut closed: HashMap<Puzzle, Node> = HashMap::new();

	let mut max_states: usize = 0;
	let mut moves_evaluated: usize = 0;

	opened.insert(puzzle, Node { previous: None, depth: 0 });

	while opened.len() > 0 {
		let state = select_best(&opened);
		let (state, node) = opened.remove_entry(&state).unwrap();
		let state_clone = state.clone();
		closed.insert(state, node).unwrap();
		let (state, node) = closed.get_key_value(&state_clone).unwrap();

		// Final state
		if distance(state) == 0 {
			println!("Solution found");
			reconstruct(&closed, state);
			println!("Maximum number of simultaneous states : {}", max_states);
			println!("Number of moves evaluated             : {}", moves_evaluated);
			return ;
		}

		for next in expand(state) {
			let state_from_opened = opened.get(&next);
			let state_from_closed = closed.get(&next);
			let new_depth = node.depth + 1;

			if (state_from_closed.is_none() && (state_from_opened.is_none() || (new_depth < state_from_opened.unwrap().depth)))
				|| (new_depth < state_from_closed.unwrap().depth) {
				opened.insert(next, Node {
					previous: Some(state),
					depth: new_depth
				});
			}
		}

		moves_evaluated += 1;
		max_states = max_states.max(opened.len());
	}

	println!("No solution");
}

fn n_puzzle(filename: String) {
	let puzzle = parse(filename);
	solve(puzzle);
}

fn main() {
	for arg in env::args().skip(1) {
		n_puzzle(arg)
	}
}