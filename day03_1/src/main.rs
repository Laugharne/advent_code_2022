use std::fs::read_to_string;

fn main() {
	println!();	
	let mut sum: u32 = 0;

	// File `rucksacks.txt` must exist in the current path
	let lines:Vec<String> = read_lines("./rucksacks.txt");
	for line in &lines {
		if line.is_empty() { continue;}
		let length            = line.len();
		let midway            = length >> 1;
		let left              = &line[0..midway];
		let right             = &line[midway..];
		let mut priority: u32 = 0;

		if let Some(common_char) = find_first_common_char(&left, &right) {
			priority = get_priority(common_char);
			println!("{}\t{}\t{} ({})", left, right, priority, common_char);
			sum += priority;
		} else {
			println!("Aucun caractère commun trouvé.");
		}

	}

	println!("\nSum of the priorities = {}", sum);

	println!();

}


fn find_first_common_char(s1: &str, s2: &str) -> Option<char> {
	for char1 in s1.chars() {
		if s2.contains(char1) {
			return Some(char1);
		}
	}
	None
}


fn get_priority(item: char) -> u32 {
	let mut r = item as u32;
	if r >= 97 {
		r = r -97 + 1;
	} else {
		r = r - 65 + 27;
	}
	r

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
// Note that in both examples above,
// we must convert the &str reference returned from lines() to the owned type String,
// using .to_string() and String::from respectively.

