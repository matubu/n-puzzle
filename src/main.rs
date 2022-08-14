#![feature(binary_heap_retain)]
use std::{env, fs};
use std::rc::Rc;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

type Puzzle = Vec<Vec<usize>>;

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
	puzzle: Rc<Puzzle>,
	previous: Option<Rc<Puzzle>>,
	cost: usize,
	distance: usize,
	pos: (usize, usize)
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
		// (other.distance * 3 + other.cost * 2).cmp(&(self.distance * 3 + self.cost * 2))
		(other.distance * 2 + other.cost).cmp(&(self.distance * 2 + self.cost))
		// (other.distance + other.cost).cmp(&(self.distance + self.cost))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compute_distance(puzzle: &Puzzle, goal: &Vec<(usize, usize)>) -> usize {
	let mut dist: usize = 0;

	for (y, row) in puzzle.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			let target = goal[*cell];
			dist += manhattan(target, (x, y));
		}
	}

	dist
}

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

// - If puzzle is unsolvable -> inform the user and exit

// - Total number of states ever selected in the "opened" set (complexity in time)
// - Maximum number of states ever represented in memory at the same time
// during the search (complexity in size)
// - Number of moves to solve the puzzle
// - The sequence of states to solve the puzzle

fn expand(state: Rc<State>, goal: &Vec<(usize, usize)>) -> Vec<State> {
	let mut possibles_states = Vec::new();
	let size = (*state).puzzle.len();

	let mut add_state = |ox: isize, oy: isize| {
		let x = (*state).pos.0 as isize + ox;
		let y = (*state).pos.1 as isize + oy;

		if     (x < 0)
			|| (y < 0)
			|| (x >= size as isize)
			|| (y >= size as isize) {
			return ;
		}

		let mut new_puzzle = (*state.puzzle).clone();
		new_puzzle[(*state).pos.1][(*state).pos.0] = new_puzzle[y as usize][x as usize];
		new_puzzle[y as usize][x as usize] = 0;

		possibles_states.push(State {
			previous: Some(Rc::clone(&(*state).puzzle)),
			cost: (*state).cost + 1,
			distance: compute_distance(&new_puzzle, goal),
			pos: (x as usize, y as usize),
			puzzle: Rc::new(new_puzzle)
		});
	};

	add_state(-1,  0);
	add_state( 1,  0);
	add_state( 0, -1);
	add_state( 0,  1);

	possibles_states
}

fn print_puzzle(puzzle: &Puzzle, previous: Option<(usize, usize)>, goal: &Vec<(usize, usize)>) -> Option<(usize, usize)> {
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
			match manhattan(goal[*cell], (x, y)) {
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
	println!("{}     \x1B[1m{}\x1B[0m", "    ".repeat(puzzle.len()), compute_distance(puzzle, goal));
	println!("");

	pos
}

fn reconstruct(map: HashMap<Rc<Puzzle>, Rc<State>>, final_state: Rc<Puzzle>, goal: &Vec<(usize, usize)>) {
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
		previous = print_puzzle(&(*state), previous, &goal);
	}
	println!("Number of moves                       : {}", path.len() - 1);
}

fn solve(puzzle: Puzzle) {
	println!("Solving...");

	let goal = build_spiral(puzzle.len());

	if compute_distance(&puzzle, &goal) % 2 == 1 {
		println!("[ Impossible puzzle ]");
		return ;
	}

	let mut heap: BinaryHeap<Rc<State>> = BinaryHeap::new();
	let mut vis: HashMap<Rc<Puzzle>, Rc<State>> = HashMap::new();

	let mut max_states: usize = 0;
	let mut moves_evaluated: usize = 0;

	let start = Rc::new(State {
		previous: None,
		cost: 0,
		distance: compute_distance(&puzzle, &goal),
		pos: find_empty(&puzzle),
		puzzle: Rc::new(puzzle)
	});
	vis.insert(start.puzzle.clone(), start.clone());
	heap.push(start);

	while let Some(state) = heap.pop() {
		// Check if it was not replaced
		if state.cost > (*vis.get(&state.puzzle).unwrap()).cost {
			continue ;
		}

		// Final state
		if state.distance == 0 {
			println!("[ Solution found ]");
			reconstruct(vis, (*state).puzzle.clone(), &goal);
			println!("Maximum number of simultaneous states : {}", max_states);
			println!("Number of moves evaluated             : {}", moves_evaluated);
			return ;
		}

		for next in expand(state, &goal) {
			if vis.contains_key(&next.puzzle) && next.cost >= (*vis.get(&next.puzzle).unwrap()).cost {
				continue ;
			}

			let rc_next = Rc::new(next);

			vis.insert(rc_next.puzzle.clone(), rc_next.clone());
			heap.push(rc_next);
		}

		moves_evaluated += 1;
		max_states = max_states.max(heap.len());
	}

	println!("[ No solution found ]");
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