use std::{env, fs};
use std::rc::Rc;
use std::collections::{HashMap, BinaryHeap};
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
		// (self.cost * 4 + self.distance).cmp(&(other.cost * 4 + other.distance))
		/*other.cost.cmp(&self.cost)
			.then_with(|| */other.distance.cmp(&self.distance)/*)
			.reverse()*/
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// TODO remove unwrap... and create better error handling

fn compute_distance(puzzle: &Puzzle) -> usize {
	let mut dist: usize = 0;

	for (y, row) in puzzle.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if *cell == 0 {
				continue ;
			}
			let target = get_spiral_position(*cell, puzzle.len());
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
	panic!("Not found");
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


// - If puzzle is unsolvable -> inform the user and exit

// - Total number of states ever selected in the "opened" set (complexity in time)
// - Maximum number of states ever represented in memory at the same time
// during the search (complexity in size)
// - Number of moves to solve the puzzle
// - The sequence of states to solve the puzzle

fn expand(state: Rc<State>) -> Vec<State> {
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
			distance: compute_distance(&new_puzzle),
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

fn reconstruct(map: HashMap<Rc<Puzzle>, Rc<State>>, final_state: Rc<Puzzle>) {
	let mut curr: Option<Rc<Puzzle>> = Some(final_state);
	let mut path = Vec::<Rc<Puzzle>>::new();

	println!("Reconstructing...");
	while let Some(state) = curr {
		path.push(state.clone());
		curr = map.get(&state).unwrap().previous.clone();
	}
	let mut previous: Option<(usize, usize)> = None;
	for (i, state) in path.iter().rev().enumerate() {
		println!("step {i}:");
		previous = print_puzzle(&(*state), previous);
	}
	println!("Number of moves                       : {}", path.len());
}

fn solve(puzzle: Puzzle) {
	println!("Solving...");

	let mut heap: BinaryHeap<Rc<State>> = BinaryHeap::new();
	let mut vis: HashMap<Rc<Puzzle>, Rc<State>> = HashMap::new();

	let mut max_states: usize = 0;
	let mut moves_evaluated: usize = 0;

	let start = Rc::new(State {
		previous: None,
		cost: 0,
		distance: compute_distance(&puzzle),
		pos: find_empty(&puzzle),
		puzzle: Rc::new(puzzle)
	});
	vis.insert(start.puzzle.clone(), start.clone());
	heap.push(start);

	while let Some(state) = heap.pop() {
		// Final state
		if state.distance == 0 {
			println!("Solution found");
			reconstruct(vis, (*state).puzzle.clone());
			println!("Maximum number of simultaneous states : {}", max_states);
			println!("Number of moves evaluated             : {}", moves_evaluated);
			return ;
		}

		for next in expand(state) {
			if vis.contains_key(&next.puzzle) {
				// if next.cost < (*vis.get(&next.puzzle).unwrap()).cost {
					// TODO replace
					// println!("should replace");
					// *(Rc::get_mut(vis.get_mut(&next.puzzle).unwrap()).unwrap()) = next.clone();
					// TODO avoid duplicate in heap
					// heap.push(vis.get(&next.puzzle).unwrap().clone());
				// }
				continue ;
			}

			let rc_next = Rc::new(next);

			vis.insert(rc_next.puzzle.clone(), rc_next.clone());
			heap.push(rc_next);

	// 		let state_from_opened = opened.get(&next);
	// 		let state_from_closed = closed.get(&next);
	// 		let new_depth = node.depth + 1;

	// 		if (state_from_closed.is_none() && (state_from_opened.is_none() || (new_depth < state_from_opened.unwrap().depth)))
	// 			|| (new_depth < state_from_closed.unwrap().depth) {
	// 			opened.insert(RefCell::new(next), State {
	// 				previous: Some(RefCell::clone(state)),
	// 				depth: new_depth
	// 			});
	// 		}
		}

		moves_evaluated += 1;
		max_states = max_states.max(vis.len());
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