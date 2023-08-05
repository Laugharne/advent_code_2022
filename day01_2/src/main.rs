use std::fs::read_to_string;

struct Elf {
	number  : u32,
	calories: u32
}

fn main() {
	let mut elves: Vec<Elf>  = Vec::new();
	let mut food: bool;

	let mut elf: Elf = Elf {
		number: 1,
		calories: 0
	};

	// File `Elf.txt` must exist in the current path
	let lines:Vec<String> = read_lines("./elves.txt");
	for element in &lines {
		food = !element.is_empty();
		if food == true {
			//println!("{}", element);
			match element.parse::<u32>() {
				Ok(n) => {
					elf.calories += n;
				},
				Err(e)=> {
					eprintln!("Conversion error : {}", e);
				}
			}
		} else {
			elves.push( elf_with_food( &mut elf));
		}

	}

	if elf.calories > 0 {
		elves.push( elf_with_food( &mut elf));
	}

	println!();

	let mut max_calories = 0;

	for e in &elves {
		//println!("Elf # {}\t; {}\tcalories", e.number, e.calories);
		if e.calories <= max_calories {continue;}
		max_calories = e.calories; // update the maximum quantity of food carried by an  elf
	}

	println!("\nMost total Calories carrying by an elf = {}", max_calories);

	elves.sort_by(
		|a, b|
		b.calories.cmp( &a.calories)
	);

	let mut max_calories = 0;
	let mut loop_counter:i32 = 0;
	for e in &elves {
		println!("Elf # {}\t; {}\tcalories", e.number, e.calories);
		max_calories += e.calories;
		loop_counter+=1;
		if loop_counter >= 3 { break;}
	}

	println!("\nCalories carrying by the TOP 3 elves = {}", max_calories);
	
}




fn elf_with_food( elf: &mut Elf) -> Elf {
	let new_elf: Elf = Elf {
		number  : elf.number,
		calories: elf.calories
	};

	//println!("    Elf # {} ; {} calories\n", new_elf.number, new_elf.calories);

	elf.calories  = 0;
	elf.number   += 1;

	new_elf
}


// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

// Since the method lines() returns an iterator over the lines in the file,
// we can also perform a map inline and collect the results,
// yielding a more concise and fluent expression.
/*
fn read_lines(filename: &str) -> Vec<String> {
	read_to_string(filename) 
		.unwrap()  // panic on possible file-reading errors
		.lines()  // split the string into an iterator of string slices
		.map(String::from)  // make each slice into a string
		.collect()  // gather them together into a vector
}
*/
// Note that in both examples above,
// we must convert the &str reference returned from lines() to the owned type String,
// using .to_string() and String::from respectively.


fn read_lines(filename: &str) -> Vec<String> {
	let mut result = Vec::new();

	for line in read_to_string(filename).unwrap().lines() {
		result.push(line.to_string())
	}

	result
}










/*
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

// Here we pass ownership of the open File to a `BufReader` struct. `BufReader` uses an internal buffer to reduce intermediate allocations.
//
// We also update `read_lines` to return an iterator instead of allocating new String objects in memory for each line.

fn main() {
	// File `Elf.txt` must exist in the current path
	if let Ok(lines) = read_lines("./Elf.txt") {
		// Consumes the iterator, returns an (Optional) String
		for line in lines {
			if let Ok(ip) = line {
				println!("{}", ip);
			}
		}
	}
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

*/