#![feature(binary_heap_retain)]

use rustc_hash::FxHashMap;

use std::{env, fs};
use std::rc::Rc;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

type DistanceFn = fn((usize, usize), (usize, usize)) -> usize;

type Puzzle = Vec<Vec<usize>>;

#[derive(Clone, Eq, PartialEq)]
struct State {
	puzzle: Rc<Puzzle>,
	previous: Option<Rc<Puzzle>>,
	cost: usize,
	distance: usize,
	pos: (usize, usize)
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
		(other.distance + other.cost).cmp(&(self.distance + self.cost))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[inline]
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
fn out_of_place(target: (usize, usize), pos: (usize, usize)) -> usize {
	if target == pos { 0 } else { 4 }
}
fn euclidean(target: (usize, usize), pos: (usize, usize)) -> usize {
	abs_diff(target.0, pos.0).pow(2) + abs_diff(target.1, pos.1).pow(2)
}

#[inline]
fn compute_distance(puzzle: &Puzzle, goal: &Vec<(usize, usize)>, distance_fn: DistanceFn) -> usize {
	let mut dist: usize = 0;

	for (y, row) in puzzle.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			let target = goal[*cell];
			dist += distance_fn(target, (x, y));
		}
	}

	dist
}

#[inline]
fn find_empty(puzzle: &Puzzle) -> (usize, usize) {
	for (y, row) in puzzle.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if *cell == 0 {
				return (x, y);
			}
		}
	}
	panic!("No empty cell found");
}

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
		}
	}

	vec
}

fn build_spiral(n: usize) -> Vec<(usize, usize)> {
	let mut step_size = n;
	let mut x: usize = 0;
	let mut y: usize = 0;
    let mut ret = Vec::with_capacity(n * n);

    ret.push((if n % 2 == 0 { n / 2 - 1 } else { n / 2 }, n / 2));

	while step_size > 1 {
        let mut add_line = |dx: isize, dy: isize, line_length: usize| {
            if ret.len() >= (n * n) {
                return ;
            }
            for _ in 0..line_length-1 {
                ret.push((x, y));
                x = (x as isize + dx) as usize;
                y = (y as isize + dy) as usize;
            }
        };

        add_line( 1,  0, step_size);
        add_line( 0,  1, step_size);
        add_line(-1,  0, step_size);
        add_line( 0, -1, step_size);

        step_size -= 2;
        x += 1;
        y += 1;
	}

    ret
}

#[inline]
fn print_puzzle(puzzle: &Puzzle, previous: Option<(usize, usize)>,
				goal: &Vec<(usize, usize)>, distance_fn: DistanceFn) -> Option<(usize, usize)> {
	let mut pos: Option<(usize, usize)> = None;

	for (y, row) in puzzle.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if *cell == 0 {
				print!("\x1B[1;91m [0]\x1B[0m ");
				pos = Some((x, y));
			} else {
				if let Some(prev) = previous {
					if x == prev.0 && y == prev.1 {
						print!("\x1B[1;92m");
					}
				}
				print!(" {:2}\x1B[0m  ", cell);
			}
		}
		for (x, cell) in row.iter().enumerate() {
			match distance_fn(goal[*cell], (x, y)) {
				0 => print!("\x1B[104m"),
				1 => print!("\x1B[106m"),
				2 => print!("\x1B[102m"),
				3 => print!("\x1B[103m"),
				_ => print!("\x1B[101m"),
			}
			print!("  \x1B[0m");
		}
		println!("\x1B[0m");
	}
	println!("{}     \x1B[1m{}\x1B[0m", "    ".repeat(puzzle.len()), compute_distance(puzzle, goal, distance_fn));
	println!("");

	pos
}

fn reconstruct(map: FxHashMap<Rc<Puzzle>, Rc<State>>, final_state: Rc<Puzzle>,
				goal: &Vec<(usize, usize)>, distance_fn: DistanceFn) {
	let mut curr: Option<Rc<Puzzle>> = Some(final_state);
	let mut path = Vec::<Rc<Puzzle>>::new();

	println!("Reconstructing...");
	while let Some(state) = curr {
		path.push(state.clone());
		curr = map.get(&state).unwrap().previous.clone();
	}
	let mut previous: Option<(usize, usize)> = None;
	for (i, state) in path.iter().rev().enumerate() {
		match i {
			0 => println!("initial state"),
			n => println!("step {}:", n)
		}
		previous = print_puzzle(&(*state), previous, &goal, distance_fn);
	}
	println!("Number of moves                       : {}", path.len() - 1);
}

fn solve(puzzle: Puzzle, distance_fn: DistanceFn) {
	println!("Solving...");

	let size = puzzle.len();
	let goal = build_spiral(size);

	// if compute_distance(&puzzle, &goal, manhattan) % 2 == 1 { // Does not work (always possible)
	// 	println!("predict: \x1B[1;91mimpossible\x1B[0m");
	// } else {
	// 	println!("predict: \x1B[1;92mpossible\x1B[0m");
	// }
	// return ;

	let mut heap: BinaryHeap<Rc<State>> = BinaryHeap::new();
	let mut vis: FxHashMap<Rc<Puzzle>, Rc<State>> = FxHashMap::default();

	let mut max_states: usize = 0;
	let mut moves_evaluated: usize = 0;
	let mut moves_skipped: usize = 0;

	let start = Rc::new(State {
		previous: None,
		cost: 0,
		distance: compute_distance(&puzzle, &goal, distance_fn),
		pos: find_empty(&puzzle),
		puzzle: Rc::new(puzzle)
	});
	vis.insert(start.puzzle.clone(), start.clone());
	heap.push(start);

	while let Some(state) = heap.pop() {
		// Check if it was not replaced
		if state.cost > (*vis.get(&state.puzzle).unwrap()).cost {
			moves_skipped += 1;
			continue ;
		}

		// Final state
		if state.distance == 0 {
			reconstruct(vis, (*state).puzzle.clone(), &goal, distance_fn);
			println!("Number of moves evaluated             : {}", moves_evaluated);
			println!("Number of moves skipped               : {}", moves_skipped);
			println!("Maximum number of simultaneous states : {}", max_states);
			return ;
		}

		// Add neighbours states
		let mut add_state = |ox: isize, oy: isize| {
			let new_x = (*state).pos.0 as isize + ox;
			let new_y = (*state).pos.1 as isize + oy;

			// Check if the empty cell does not end up outside of the board
			if (new_x < 0) || (new_y < 0)
				|| (new_x >= size as isize) || (new_y >= size as isize) {
				return ;
			}

			// Generate the new puzzle by swapping the two value
			let mut new_puzzle = (*state.puzzle).clone();
			new_puzzle[(*state).pos.1][(*state).pos.0] = new_puzzle[new_y as usize][new_x as usize];
			new_puzzle[new_y as usize][new_x as usize] = 0;
			let new_puzzle = Rc::new(new_puzzle);

			let new_cost = (*state).cost + 1;

			if vis.contains_key(&new_puzzle) && new_cost >= (*vis.get(&new_puzzle).unwrap()).cost {
				return ;
			}

			let next = Rc::new(State {
				previous: Some(Rc::clone(&(*state).puzzle)),
				cost: new_cost,
				distance: compute_distance(&new_puzzle, &goal, distance_fn),
				pos: (new_x as usize, new_y as usize),
				puzzle: new_puzzle
			});

			vis.insert(next.puzzle.clone(), next.clone());
			heap.push(next);
		};

		add_state(-1,  0);
		add_state( 1,  0);
		add_state( 0, -1);
		add_state( 0,  1);

		max_states = max_states.max(heap.len());
		moves_evaluated += 1;
	}

	println!("\x1B[1;91mNo solution found\x1B[0m");
}

fn n_puzzle(filename: String, distance_fn: DistanceFn) {
	let puzzle = parse(filename);
	solve(puzzle, distance_fn);
}

fn help(error: &str) {
	println!("\x1B[91mError\x1B[0m: {error}");
	println!("Flags:");
	println!("  --euclidean, -e         use euclidean distance (default)");
	println!("  --out_of_place, -o      use out_of_place distance");
	println!("  --manhattan, -m         use manhattan distance");
	println!("Usage: n-puzzle [Flags] [Files]");
	std::process::exit(1);
}

fn main() {
	let args = env::args().skip(1);
	let (flags, files): (Vec<String>, Vec<String>) = args.partition(|arg| arg.starts_with("-"));
	let mut distance_fn: DistanceFn = euclidean;

	for flag in flags {
		match flag.as_str() {
			"--euclidean" | "-e" => {
				distance_fn = euclidean;
			},
			"--out_of_place" | "-o" => {
				distance_fn = out_of_place;
			},
			"--manhattan" | "-m" => {
				distance_fn = manhattan;
			}
			_ => {
				help("Flag not supported");
			}
		}
	}


	if files.len() < 1 {
		help("Should at least contain one file");
	}
	for file in files {
		n_puzzle(file, distance_fn);
	}
}