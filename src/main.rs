use std::env;
use std::fs;

struct Node {
	index: usize,
	candidate: [u32; 81],
	children: [usize; 9], // store index of children in vec
}

fn main() {
	// Command line arguments
	let argv: Vec<String> = env::args().collect();
	if argv.len() != 2 && argv.len() != 3 {
		print_err();
		return;
	}

	// Check if print partials should be run
	let mut print_partials: Option<u64> = None;
	if argv.len() == 3 {
		if argv[2][..2] == *"-p" {
			let delay: Result<u64, _> = argv[2][2..].parse::<u64>();
			if delay.is_err() {
				print_err();
				return;
			}
			print_partials = delay.ok();
		} else {
			print_err();
			return;
		}
	}

	let mut node_list: Vec<Node> = Vec::new();
	let mut puzzle: [u32; 81] = [0; 81];
	parse(&mut puzzle, &argv[1]);

	let root = Node {
		index: 0,
		candidate: puzzle,
		children: [0; 9],
	};
	node_list.push(root);

	let solution = backtrack(0, &mut node_list, print_partials);

	match solution {
		Ok(x) => print_solution(&node_list[x].candidate),
		Err(_) => println!("Unable to solve puzzle!"),
	}
}

// Recursively solve the puzzle
fn backtrack(
	parent_ind: usize,
	node_list: &mut Vec<Node>,
	delay: Option<u64>,
) -> Result<usize, ()> {
	let mut to_change: usize = 0;

	// Print the partial solutions and add delay
	if let Some(del) = delay {
		std::thread::sleep(std::time::Duration::from_millis(del));
		print_solution(&node_list[parent_ind].candidate);
	}

	// Base cases
	if reject(&node_list[parent_ind].candidate) {
		return Err(());
	} else if accept(&node_list[parent_ind].candidate) {
		return Ok(parent_ind);
	}

	// recursively solve the puzzle
	let mut p: Result<usize, ()>;
	let mut s: Result<usize, ()> = Ok(first(parent_ind, node_list, &mut to_change));

	// create next 8 children
	while s.is_ok() {
		p = backtrack(s.unwrap(), node_list, delay);
		if p.is_ok() {
			return p;
		}
		p = s;
		s = next(parent_ind, p.unwrap(), node_list, &to_change);
	}

	Err(())
}

// Generate first child of parent
fn first(parent_ind: usize, node_list: &mut Vec<Node>, to_change: &mut usize) -> usize {
	let empty_arr: [usize; 9] = [0; 9];
	let mut empty_puzzle: [u32; 81] = [0; 81];

	let mut is_found: bool = false;

	for (i, n) in node_list[parent_ind].candidate.iter_mut().enumerate() {
		empty_puzzle[i] = *n;
		if *n == 0 && !is_found {
			*to_change = i as usize;
			is_found = true;
			empty_puzzle[i] = 1;
		}
	}
	let child = Node {
		index: 0,
		candidate: empty_puzzle,
		children: empty_arr,
	};
	let length = node_list[parent_ind].index;
	let mut parent = &mut node_list[parent_ind];
	parent.children[parent.index] = length;
	node_list[parent_ind].children[0] = node_list.len();
	node_list[parent_ind].index = 1;
	node_list.push(child);

	node_list.len() - 1
}

// Generate next child after prev
fn next(
	parent_ind: usize,
	prev_ind: usize,
	node_list: &mut Vec<Node>,
	to_change: &usize,
) -> Result<usize, ()> {
	if node_list[parent_ind].index > 8 {
		return Err(());
	}

	let empty_arr: [usize; 9] = [0; 9];
	let mut empty_puzzle: [u32; 81] = [0; 81];

	empty_puzzle[..81].copy_from_slice(&node_list[prev_ind].candidate[..81]);

	empty_puzzle[*to_change] += 1;

	let child = Node {
		index: 0,
		candidate: empty_puzzle,
		children: empty_arr,
	};

	let length: usize = node_list.len();
	let parent: &mut Node = &mut node_list[parent_ind];
	parent.children[parent.index] = length;
	parent.index += 1;
	node_list.push(child);

	Ok(length) // this works out to be the index of child
}

// Check if everysquare is full
fn accept(puzzle: &[u32; 81]) -> bool {
	for i in puzzle {
		if *i == 0 {
			return false;
		}
	}

	true
}

// Check if candidate is still valid
fn reject(puzzle: &[u32; 81]) -> bool {
	let mut counter: [u32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // first element is for blanks

	// Check horizontal rows
	for i in 0..9 {
		for j in 0..9 {
			counter[puzzle[(i * 9 + j) as usize] as usize] += 1;
		}

		for j in counter.iter_mut().skip(1) {
			if *j > 1 {
				return true;
			}

			*j = 0;
		}
	}

	// check vertical rows
	for i in 0..9 {
		for j in 0..9 {
			counter[puzzle[(j * 9 + i) as usize] as usize] += 1;
		}

		for j in counter.iter_mut().skip(1) {
			if *j > 1 {
				return true;
			}

			*j = 0;
		}
	}

	// check squares
	// traverse rows
	for i in 0..3 {
		// traverse columns
		for j in 0..3 {
			let offset: usize = (i * 27) + (j * 3);
			// traverse row in square
			for k in 0..3 {
				counter[puzzle[offset + k * 9] as usize] += 1;
				counter[puzzle[offset + k * 9 + 1] as usize] += 1;
				counter[puzzle[offset + k * 9 + 2] as usize] += 1;
			}

			for k in counter.iter_mut().skip(1) {
				if *k > 1 {
					return true;
				}

				*k = 0;
			}
		}
	}

	false
}

// Output puzzle
fn print_solution(puzzle: &[u32; 81]) {
	for i in 0..3 {
		let mut offset = i * 27;
		println!("+-------+-------+-------+");
		for _ in 0..3 {
			println!(
				"| {} {} {} | {} {} {} | {} {} {} |",
				puzzle[offset],
				puzzle[offset + 1],
				puzzle[offset + 2],
				puzzle[offset + 3],
				puzzle[offset + 4],
				puzzle[offset + 5],
				puzzle[offset + 6],
				puzzle[offset + 7],
				puzzle[offset + 8]
			);
			offset += 9;
		}
	}
	println!("+-------+-------+-------+");
}

fn print_err() {
	println!("Using rust_sudoku <FIlENAME> [options]");
	println!("\nOptions:");
	println!("	-p<delay>	Print the partial solutions (will add <delay> ms to make output readable");
}

// Read input from file
fn parse(puzzle: &mut [u32; 81], filename: &String) {
	let contents = fs::read_to_string(filename).expect("Error reading file");
	let mut i = 0;
	let mut j = 0;
	while j < 81 {
		let c = contents.chars().nth(i).unwrap();
		if c.is_whitespace() || c == ',' {
			i += 1;
			continue;
		}
		let oc = c.to_digit(10).unwrap_or(0) as u32;
		puzzle[j] = oc;
		j += 1;
		i += 1;
	}
}
