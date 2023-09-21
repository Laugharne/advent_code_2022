use std::fs::read_to_string;

struct Trace {
	symbol:  char,
	layer: u16,
}

fn main() {

	let mut x_min: i32 = 0;
	let mut x_max: i32 = 0;

	let mut y_min: i32 = 0;
	let mut y_max: i32 = 0;

	// Map dimensions
	let map_nx: usize;
	let map_ny: usize;

	// Absolute value of start point position
	let x_start: u32;
	let y_start: u32;

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


	let mut map: Vec<Vec<Trace>> = Vec::new();

	for my in 00..map_ny {
		map.push(Vec::new());
		for _x in 00..map_nx {
			map[my].push( Trace {
				symbol: '.',
				layer: 0
			});
		}
	}
	/*for mx in 00..map_nx {
		map.push(Vec::new());
		for _y in 00..map_ny {
			map[mx].push( Trace {
				symbol: '.',
				layer: 0
			});
		}
	}*/

	println!();

	let nn_movements: i32 = movements.len() as i32;
	let mut x_h: u32 = x_start;
	let mut y_h: u32 = y_start;
	let mut x_t: u32 = x_start;
	let mut y_t: u32 = y_start;

	for mv in -1..nn_movements {
		if mv < 0 {
			println!("== Initial State ==");
			map[y_start as usize][x_start as usize].symbol = 's';
			display_map( &map);
		} else {
			let movement: &Move = &movements[mv as usize];
			println!("== {} {} ==", movement.direction, movement.step);

			let mut x_h_new = x_h;
			let mut y_h_new = y_h;
			let mut x_t_new = x_t;
			let mut y_t_new = y_t;

			for step in 0..movement.step {
				match movement.direction {
					'U' => { y_h_new -=1; },
					'D' => { y_h_new +=1; },
					'L' => { x_h_new -=1; },
					'R' => { x_h_new +=1; },
					_ => {},
				}

				// TODO
				
				map[y_h as usize][x_h as usize].symbol = '.';
				map[y_t as usize][x_t as usize].symbol = '.';

				map[y_start as usize][x_start as usize].symbol = 's';
				map[y_t_new as usize][x_t_new as usize].symbol = 'T';  // `T` covers `s`
				map[y_h_new as usize][x_h_new as usize].symbol = 'H';  // `H` covers `T` and `s`

				x_h = x_h_new;
				y_h = y_h_new;
				x_t = x_t_new;
				y_t = y_t_new;
		
				// Display trace `H` & `T`
				display_map( &map);
			
			}

		}

	}

	println!("\n== TRACE ==");
	let mut visited: u32 = 0;

	map.iter().for_each(|line| {
		line.iter().for_each(|cell| {
			let mark: char;	
			match cell.layer {
				0 => {mark = '.';},
				_ => {mark = '#'; visited += 1;},
			}
			print!(" {}", mark);
		});
		println!();
	});

	println!();
	println!("Visited positions = {}", visited);
	println!();

}


fn display_map(map: &Vec<Vec<Trace>>) {
	map.iter().for_each(|line| {
		line.iter().for_each(|cell| {
			print!(" {}", cell.symbol);
		});
		println!();
	});
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

