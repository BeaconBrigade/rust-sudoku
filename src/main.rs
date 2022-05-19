use std::env;
use std::fs;

struct Node<'a>
{
	puzzle: [i32; 81],
	children: [&'a mut Node<'a>; 9],
}

fn main() 
{
	// Command line arguments
	let argv: Vec<String> = env::args().collect();
	if argv.len() != 2 {
		println!("Using {} <FIlENAME>", argv[0]);
		return;
	}

	let mut puzzle: [i32; 81] = [0; 81];
	parse(&mut puzzle, &argv[1]);

	for i in 0..81
	{
		println!("num {} = {}", i, puzzle[i]);
	}
}

// Read input from file
fn parse(puzzle: &mut [i32; 81], filename: &String)
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
		puzzle[j] = c.to_digit(10).unwrap_or(0) as i32;
		j += 1;
		i += 1;
	}
}

