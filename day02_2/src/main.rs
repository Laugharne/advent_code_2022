use std::fs::read_to_string;

fn main() {
	let mut total_score: i32 = 0;

	// File `rounds.txt` must exist in the current path
	let lines:Vec<String> = read_lines("./rounds.txt");
	for line in &lines {
		//-println!("{}", line);	
		if line.is_empty() { continue;}
		let result: Vec<&str> = split_string(&line, ' ');
		//println!("A: {}\tB:{}", result[0], result[1]);
		total_score += result_for_a_round(result[0], result[1]);
	}

	println!("Final Total Score = {}", total_score);	

}

fn get_index_from_abc(input: &str) -> Option<u8> {
	if let Some(first_char) = input.chars().next() {
		let r = first_char as u8;
		return Some(r - 65);
	}
	None
}

fn get_index_from_xyz(input: &str) -> Option<u8> {
	if let Some(first_char) = input.chars().next() {
		let r = first_char as u8;
		return Some(r - 88);
	}
	None
}


fn result_for_a_round( abc: &str, xyz: &str) -> i32 {

	// X means you need to lose,
	// Y means you need to end the round in a draw,
	// and Z means you need to win.
	let compute_index_response: [[i32; 3]; 3] = [
		// (XYZ)
		// ROCK, PAPER, SCISSORS
		[2, 0, 1],	// (ABC) ROCK
		[0, 1, 2],	// (ABC) PAPER
		[1, 2, 0],	// (ABC) SCISSORS
	];

	let result_per_round: [[i32; 3]; 3] = [
		// (XYZ)
		// ROCK, PAPER, SCISSORS
		[3, 6, 0],	// (ABC) ROCK
		[0, 3, 6],	// (ABC) PAPER
		[6, 0, 3],	// (ABC) SCISSORS
	];

	//println!("ABC: {}\tXYZ: {}", abc, xyz);

	let mut index_abc: i32 = 0;
	let mut index_xyz: i32 = 0;
	if let Some(index) = get_index_from_abc(&abc) {
		index_abc = index.into();
	} else {
		println!("La chaîne `abc` est vide.");
	}

	if let Some(index) = get_index_from_xyz(&xyz) {
		index_xyz = index.into();
	} else {
		println!("La chaîne `xyz` est vide.");
	}

	let index_respone = compute_index_response[index_abc as usize][index_xyz as usize];

	// compute
	let score_per_round = result_per_round[index_abc as usize][index_respone as usize]
		+ (index_respone + 1);

	println!("Round : {}", score_per_round);
	
	score_per_round

}


fn split_string(input: &str, separator: char) -> Vec<&str> {
	input.split(separator).collect()
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

/*
fn read_lines(filename: &str) -> Vec<String> {
	let mut result = Vec::new();

	for line in read_to_string(filename).unwrap().lines() {
		result.push(line.to_string())
	}

	result
}
*/