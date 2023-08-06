use std::fs::read_to_string;
use in_range::in_range_u32;

fn main() {
	println!();	

	let mut sum: u32          = 0;
	let mut overlap: u32      = 0;
	let mut line_counter: u32 = 0;

	// File `pairs.txt` must exist in the current path
	let lines:Vec<String> = read_lines("./pairs.txt");
	for line in &lines {
		if line.is_empty() { continue;}
		let sections:   Vec<&str> = line.split(',').collect();
		let first_str:  Vec<&str> = sections[0].split('-').collect();
		let second_str: Vec<&str> = sections[1].split('-').collect();

		let mut a: u32 = 0;
		let mut b: u32 = 0;
		let mut c: u32 = 0;
		let mut d: u32 = 0;

		match first_str[0].parse::<u32>() {
			Ok(parsed_num) => { a = parsed_num;}
			Err(_) => {	println!("Impossible de convertir la chaîne en u32.");}
		}
		match first_str[1].parse::<u32>() {
			Ok(parsed_num) => { b = parsed_num;}
			Err(_) => {	println!("Impossible de convertir la chaîne en u32.");}
		}
		match second_str[0].parse::<u32>() {
			Ok(parsed_num) => { c = parsed_num;}
			Err(_) => {	println!("Impossible de convertir la chaîne en u32.");}
		}
		match second_str[1].parse::<u32>() {
			Ok(parsed_num) => { d = parsed_num;}
			Err(_) => {	println!("Impossible de convertir la chaîne en u32.");}
		}

		//println!("{} - {}\t{} - {}", a, b, c, d);

		// ...a..b..   
		// .c......d   
		//
		// .a......b   
		// ...c..d..   
		if ((a >= c) && (b <= d)) || ((c >= a) && (d <= b)) {
			sum += 1;
		}

		// .a..b....   
		// ...c..d..   
		//
		// ...a..b..   
		// .c..d....   
		if	in_range_u32(a, c, d)
			||
			in_range_u32(b, c, d)
			||
			in_range_u32(c, a, b)
			||
			in_range_u32(d, a, b)
		{
			overlap += 1;
		}

		line_counter += 1;

	}

	println!("\nNumber of assignment pairs with one range fullly contain the other = {}", sum);
	println!("\nNumber of overlap = {}", overlap);
	println!();	

}

/*
fn in_range(value: u32, begin: u32, end: u32) -> bool {
	(value >= begin) && (value <= end)
}
*/


/*
fn split_string(input: &str, separator: char) -> Vec<&str> {
	input.split(separator).collect()
}
*/


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

