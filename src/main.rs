use std::env;
use std::fs;

#[derive(Clone)]
struct Node
{
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

	let root: Node = Node { candidate: puzzle, children: [0; 9] };

	let solution = backtrack(&root, node_list);

	match solution {
		Some(x) => print_solution(&x.candidate),
		None => println!("Unable to solve puzzle!"),
	}
}

// Recursively solve the puzzle
fn backtrack(parent : &Node, node_list: Vec<Node>) -> Option<Node>
{
	let mut to_change: usize = 0;

	// Base cases
	if reject(&parent.candidate) {
		return None;
	} else if accept(&parent.candidate) {
		return Some(parent.clone());
	}


	None
}

// Generate first child of parent
fn first(parent: &mut Node, node_list: Vec<Node>, to_change: &mut usize) -> Node
{

}

// Generate next child after prev
fn next(parent: &mut Node, prev: &Node, node_list: Vec<Node>, to_change: &usize) -> Option<Node>
{

}

// Check if everysquare is full
fn accept(puzzle: &[u32; 81]) -> bool
{
	for i in 0..81
	{
		if puzzle[i] == 0 {
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