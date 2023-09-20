use std::fs::read_to_string;


fn main() {

	let mut x_min: i32 = 0;
	let mut x_max: i32 = 0;

	let mut y_min: i32 = 0;
	let mut y_max: i32 = 0;

	// Map dimensions
	let mut map_nx: usize = 0;
	let mut map_ny: usize = 0;

	// Absolute value of start point position
	let mut x_start: u32 = 0;
	let mut y_start: u32 = 0;

	println!();

	#[derive(Debug)]
	struct Move {
		direction: char,
		step: u8,
	}

	// File `movements.txt` must exist in the current path
	let lines:Vec<String> = read_lines("./movements.txt");
	let mut movements: Vec<Move> = Vec::new();

	for line in &lines {
		if line.is_empty() { continue;}
		println!("'{}'", line);
		let words: Vec<&str> = line.split_whitespace().collect();

		movements.push( Move {
			direction : words[0].chars().next().unwrap(),
			step : words[1].parse::<u8>().unwrap()
		});

	}

	println!();

	for mv in &movements {
		println!("{:?}", mv);
	}

	println!();

	let mut x: i32 = 0;
	let mut y: i32 = 0;

	for mv in &movements {
		//println!("{:?}", mv);
		match mv.direction {
			'U' => { y -= mv.step as i32;},
			'D' => { y += mv.step as i32;},
			'L' => { x -= mv.step as i32;},
			'R' => { x += mv.step as i32;},
			_ => {}
		}

		if x < x_min { x_min = x;}
		if y < y_min { y_min = y;}
		if x > x_max { x_max = x;}
		if y > y_max { y_max = y;}
	}

	println!();
	println!("x (min, max) = {},{}", x_min, x_max);
	println!("y (min, max) = {},{}", y_min, y_max);

	map_nx = (x_max - x_min +1) as usize;
	map_ny = (y_max - y_min +1) as usize;

	x_start = (map_nx as u32) - (x_max as u32) -1;

	y_start = (map_ny as u32) - (y_max as u32) -1;

	println!();
	println!("nx,ny = {},{}", map_nx, map_ny);
	println!("ox,oy = {},{}", x_start, y_start);

	struct Trace {
		symbol:  char,
		layer: u16,
	}
	let mut map: Vec<Vec<Trace>> = Vec::new();

	for mx in 00..map_nx {
		map.push(Vec::new());
		for _y in 00..map_ny {
			map[mx].push( Trace {
				symbol: '.',
				layer: 0
			});
		}
	}


	println!();

	let nn_movements: usize = movements.len();

	for mv in 0..=nn_movements {
		let mv1: i32 = (mv as i32) - 1;
		if mv <= 0 {
			println!("\n== Initial State ==\n");
		} else {
			println!("\n== {} {} ==\n", movements[mv1 as usize].direction, movements[mv1 as usize].step);
		}
		
		map[x_start as usize][y_start as usize] = Trace {
			symbol: 's',
			layer: 0
		};

		// TODO

		for y in 0..map_ny {
			for x in 0..map_nx {
				let t = &map[x][y];
				print!(" {}", t.symbol);
			}
			println!();
		}
	
		println!();
	
	}

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

