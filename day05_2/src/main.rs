use std::fs::read_to_string;

fn main() {
	println!();	

	enum Part {
		Crates,
		Stacks,
		Move
	}

	let mut phase: Part = Part::Crates;

	let mut crates_txt: Vec<&String> = Vec::new();

	let mut number_of_stack: u32 = 0;

	let mut stack: Vec<Vec<char>> = Vec::new();

	// File `crates.txt` must exist in the current path
	let lines:Vec<String> = read_lines("./crates.txt");
	'empty: for line in &lines {

		match phase {

			Part::Crates => {
				//println!("Crates");
				if line.contains(" 1 ") {
					phase = Part::Stacks;

					for c in &crates_txt {
						println!("'{}'", c);
					}

					println!("'{}'", line);
					number_of_stack = line.len() as u32;
					let explode: Vec<&str> = line.trim().split(" ").collect();
					if let Some(last) = explode.last() {
						if let Ok(n)= (*last).parse::<u32>() {
							number_of_stack = n;
							println!("\nNbr of stacks = {}", number_of_stack);
							println!("");
						}
					}
				} else {
					// add line to buffer, to be processing later in `Stacks` part
					crates_txt.push(line);
				}
			},

			Part::Stacks => {
				//println!("Stacks");
				//println!("'{}'", line);
				if line.is_empty() {
					// '    [D]    '
					// '[N] [C]    '
					// '[Z] [M] [P]'

					// Transform text to data
					for _i in 0..number_of_stack {
						stack.push(Vec::new());
					}

					for level in crates_txt.iter().rev() {
						//println!("'{}'", level);

						// loop 0 -> number_of_stack - 1
						for st in 0..number_of_stack {
							let position: u32 = st*4 + 1;	// From stack# to position in string
							let c: char = level.chars().nth(position as usize).unwrap();
							if c == ' ' { continue;}
							stack[st as usize].push(c);
						}

					}

					//println!("{:?}", stack);

					phase = Part::Move;
					continue 'empty;
				}

			},

			Part::Move => {
				//println!("move");
				//println!("'{}'", line);
				if line.is_empty() { continue 'empty;}
				let explode: Vec<&str> = line.trim().split(" ").collect();
				let move_nn:u32        = explode[1].parse::<u32>().unwrap();
				let move_from:usize    = explode[3].parse::<usize>().unwrap() - 1;
				let move_to:usize      = explode[5].parse::<usize>().unwrap() - 1;
				println!("{} move(s)\tfrom {}\tto {}", move_nn, move_from, move_to);
				let mut tmp: Vec<char> = Vec::new();
				for _i in 0..move_nn {
					if let Some(item) = stack[move_from].pop() {
						tmp.push(item);
					}
				}

				for _i in 0..move_nn {
					if let Some(item) = tmp.pop() {
						stack[move_to].push(item);
					}
				}

			}
		}

	}

	//println!("{:?}", stack);
	println!();	
	for i in 00..number_of_stack {
		print!("{}", stack[i as usize].last().unwrap());
	}
	println!();	

	println!();	

}


// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

// Since the method lines() returns an iterator over the lines in the file,
// we can also perform a map inline and collect the results,
// yielding a more concise and fluent expression.
fn read_lines(filename: &str) -> Vec<String> {
	read_to_string(filename) 
		.unwrap()  // panic on possible file-reading errors
		.lines()  // split the string into an iterator of string slices
		.map(String::from)  // make each slice into a string
		.collect()  // gather them together into a vector
}

