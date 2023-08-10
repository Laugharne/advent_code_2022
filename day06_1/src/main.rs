use std::fs::read_to_string;

const SLICE: u32 = 4;

fn main() {
	println!();	

	// File `datastream.txt` must exist in the current path
	let lines:Vec<String> = read_lines("./datastream.txt");

	for line in &lines {
		println!("'{}'", line);

		// Transforme une String en tableau de caractères Vec<char>
		let data: Vec<char> = line.chars().collect();

		if let Some(index) = find_marker( data) {
			println!("  1st marker at {}", index);
		}
		println!();//	break;

	}

	println!();	

}


fn find_marker( data: Vec<char>) -> Option<u32> {

	let length: u32 = data.len() as u32;
	let index: u32  = SLICE-1;

	for i in index..length {
		if is_marker( &data, i) == true {
			let i1 = i+1;
			return Some(i1);
		}

	}

	None

}


fn is_marker( buffer: &Vec<char>, index: u32) -> bool {

	let mut char_counter: [u32; SLICE as usize] = [0; SLICE as usize];

	let offset: u32 = index-(SLICE-1);

	for i in 0..SLICE {
		let c = buffer[(i + offset) as usize];

		for j in 0..SLICE {
			if c != buffer[(j + offset) as usize] { continue;}
			char_counter[(i) as usize] += 1;
		}
	}

	let mut is_marker: bool = true;

	for i in 0..SLICE {
		if char_counter[i as usize] <= 1 { continue}
		is_marker = false;
		break;
	}

	is_marker
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

