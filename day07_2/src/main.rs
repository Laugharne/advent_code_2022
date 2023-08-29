use std::fs::read_to_string;
use std::collections::HashMap;


fn main() {

	println!();

	// File `commands.txt` must exist in the current path
	let lines:Vec<String> = read_lines("./commands.txt");

	let mut directory: HashMap<Vec<&str>, u32> = HashMap::new();
	let mut path_vec: Vec<&str>     = Vec::new();
	let mut total_size: u32 = 0;

	for line in &lines {
		//println!("'{}'", line);
		if line.is_empty()          { continue;}
		if line.starts_with("dir")  { continue;}
		if line.starts_with("$ ls") { continue;}

		let words: Vec<&str> = line.split_whitespace().collect();

		match words[..] {
			["$", "cd", ".."] => {
				path_vec.pop(); // $ cd ..
			},

			["$", "cd", name] => {
				path_vec.push(name); // $ cd <name>
				println!("  {:?}", path_vec);
				//let path_full: String = path_vec.join(" ");
				directory.insert( path_vec.clone(), 0);
			},

			[file_size, _file_name] => {
				let size: u32 = file_size.parse::<u32>().unwrap();
				let mut path_full: Vec<&str> = path_vec.clone();
				total_size += size;
				for _nest in 0..path_vec.len() {
					//let count: &mut u32  = directory.entry(path_vec[nest]).or_insert(0);
					let count: &mut u32  = directory.entry(path_full.clone()).or_insert(0);
					*count              += size;
					path_full.pop();
				}
			},

			_ => {}
		}

	}

	//println!("{:?}", directory);
	let sum: u32 = directory
		.clone().into_values()
		.filter(|size| *size <= 100000)
		.sum();
	
	println!();
	println!("Sum of the total sizes of directories (size <= 100000) {:?}", sum);
	println!();
	println!("Total size {:?}", total_size);
	println!();

	// exemple:
	// 48381165 (total size occupied)
	// 70000000 - 48381165 = 21618835
	// delete dir with at least 30000000 - 21618835 = 8381165

	// input:
	// 42036703
	// 70000000 - 42036703 = 27963297
	// delete dir with at least 30000000 - 27963297 = 2036703
	let threshold = 30000000 - (70000000 - total_size);

	let mut dir_to_delete: Vec<u32> = directory
		.into_values()
		.filter(|size| *size >= threshold)
		.collect()
		;

	dir_to_delete.sort();
	let size_dir_to_delete = dir_to_delete[0];
	println!();
	println!("{:?}", dir_to_delete);
	println!();
	println!("{:?}", size_dir_to_delete);
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

