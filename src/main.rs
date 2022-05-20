use std::env;
use std::fs;

#[derive(Clone)]
struct Node
{
	index: usize,
	candidate: [u32; 81],
	children: [usize; 9], // store index of children in vec
}

fn main() 
{
	// Command line arguments
	let argv: Vec<String> = env::args().collect();
	if argv.len() != 2 {
		println!("Using {} <FIlENAME>", argv[0]);
		return;
	}

	let mut node_list: Vec<Node> = Vec::new(); 
	let mut puzzle: [u32; 81] = [0; 81];
	parse(&mut puzzle, &argv[1]);

	let root = Node { index: 0, candidate: puzzle, children: [0; 9] };
	node_list.push(root);

	let solution = backtrack(0, &mut node_list);

	match solution {
		Ok(x) => print_solution(&node_list[x].candidate),
		Err(_) => println!("Unable to solve puzzle!"),
	}
}

// Recursively solve the puzzle
fn backtrack(parent_ind: usize, node_list: &mut Vec<Node>) -> Result<usize, ()>
{
	let mut to_change: usize = 0;

	// Base cases
	if reject(&node_list[parent_ind].candidate) {
		return Err(());
	} else if accept(&node_list[parent_ind].candidate) {
		return Ok(parent_ind);
	}

	// recursively solve the puzzle
	let mut p: Result<usize, ()>;
	let mut s: usize = first(parent_ind, node_list, &mut to_change);
	
	for _ in 0..8 // create next 8 children
	{
		p = backtrack(s, node_list);
		if p.is_ok() {
			return p;
		}
		p = Ok(s);
		s = next(parent_ind, p.unwrap(), node_list, &to_change);
	}


	Err(())
}

// Generate first child of parent
fn first(parent_ind: usize, node_list: &mut Vec<Node>, to_change: &mut usize) -> usize
{
	let empty_arr: [usize; 9] = [0; 9];
	let mut empty_puzzle: [u32; 81] = [0; 81];

	let mut is_found: bool = false;
	
	for i in 0..81
	{
		empty_puzzle[i] = node_list[parent_ind].candidate[i];
		if node_list[parent_ind].candidate[i] == 0 && !is_found {
			*to_change = i as usize;
			is_found = true;
			empty_puzzle[i] = 1;
		}
	}
	let child = Node { index: 0, candidate: empty_puzzle, children: empty_arr };
	
	node_list[parent_ind].children[0] = node_list.len();
	node_list[parent_ind].index = 1;
	node_list.push(child);
	
	node_list.len() - 1
}

// Generate next child after prev
fn next(parent_ind: usize, prev: usize, node_list: &mut Vec<Node>, to_change: &usize) -> usize
{
	let empty_arr: [usize; 9] = [0; 9];
	let mut empty_puzzle: [u32; 81] = [0; 81];
	
	empty_puzzle[..81].copy_from_slice(&node_list[parent_ind].candidate[..81]);

	empty_puzzle[*to_change] = node_list[prev].candidate[*to_change] + 1;
	
	let child = Node { index: 0, candidate: empty_puzzle, children: empty_arr };

	let length: usize = node_list.len();
	let parent: &mut Node = &mut node_list[parent_ind];
	parent.children[parent.index] = length;
	parent.index += 1;
	node_list.push(child);
	
	length // this works out to be the index of child
}

// Check if everysquare is full
fn accept(puzzle: &[u32; 81]) -> bool
{
	for i in puzzle
	{
		if *i == 0 {
			return false;
		}
	}

	true
}
  
// Check if candidate is still valid
fn reject(puzzle: &[u32; 81]) -> bool
{
	let mut counter: [u32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // first element is for blanks

	// Check horizontal rows
	for i in 0..9
	{
		for j in 0..9
		{
			counter[puzzle[(i * 9 + j) as usize] as usize] += 1;
		}

		for j in 1..10
		{
			if counter[j] > 1 {
				return true;
			}

			counter[j] = 0;
		}
	}

	// check vertical rows
	for i in 0..9
	{
		for j in 0..9
		{
			counter[puzzle[(j * 9 + i) as usize] as usize] += 1;
		}

		for j in 1..10
		{
			if counter[j] > 1 {
				return true;
			}

			counter[j] = 0;
		}
	}

	// check squares
	// traverse rows
	for i in 0..3
	{
		// traverse columns
		for j in 0..3
		{
			let offset: usize = (i * 27) + (j * 3);
			// traverse row in square
			for k in 0..3
			{
				counter[puzzle[offset + k * 9] as usize] += 1;
				counter[puzzle[offset + k * 9 + 1] as usize] += 1;
				counter[puzzle[offset + k * 9 + 2] as usize] += 1;
			}

			for k in 1..10
			{
				if counter[k] > 1 {
					return true;
				}

				counter[j] = 0;
			}
		}
	}

	false
}

// Output puzzle
fn print_solution(puzzle: &[u32; 81])
{
	for i in 0..9
	{
		println!("{} {} {} {} {} {} {} {} {}", puzzle[i], puzzle[i + 1], puzzle[i + 2], puzzle[i + 3], puzzle[i + 4], puzzle[i + 5], puzzle[i + 6], puzzle[i + 7], puzzle[i + 8])
	}
}

// Read input from file
fn parse(puzzle: &mut [u32; 81], filename: &String)
{
	let contents = fs::read_to_string(filename).expect("Error reading file");

	let mut i = 0;
	let mut j = 0;
	while j < 81 
	{
		let c = contents.chars().nth(i).unwrap();
		if c.is_whitespace() || c == ',' {
			i += 1;
			continue;
		}
		puzzle[j] = c.to_digit(10).unwrap_or(0) as u32;
		j += 1;
		i += 1;
	}
}
