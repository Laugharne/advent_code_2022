use std::fs::read_to_string;


fn main() {
	#[derive(Debug)]
	struct Tree {
		// Scenic score
		scenic: u32,
		// Tree height; `0` = shortest, `9` = tallest
		height: u8,
		// Visibility
		news:  u8,	// NEWS <=> North East West South
		north: bool,
		south: bool,
		west:  bool,
		east:  bool,
	}

	let mut map_tree: Vec<Vec<Tree>> = Vec::new();
	let mut map_nx: usize = 0;
	let mut map_ny: usize = 0;

	// DEBUG
	const X: usize = 2;
	const Y: usize = 3;

	println!();

	// File `map.txt` must exist in the current path
	let lines:Vec<String> = read_lines("./map.txt");
	let mut nn_line: usize = 0;

	for line in &lines {
		if line.is_empty() { continue;}
		println!("'{}'", line);

		if nn_line <= 0 {
			map_nx = line.len();
			for _x in 0..map_nx {
				map_tree.push(Vec::new());
			}
		}

		// Height map building (from ASCII data)
		for x in 0..map_nx {
			let height: u8 = line.chars().nth(x).unwrap() as u8;	// no need to convert to u8 from ascii
			let tree: Tree = Tree {
				scenic: 1,
				height: height - 48,	// 48 = ASCII "0"
				news:   0,
				north:  false,
				south:  false,
				west:   false,
				east:   false,
			};
			map_tree[x].push(tree);
		}

		nn_line += 1;
	}

	map_ny = nn_line;
	println!();
	println!("nx,ny = {:?},{:?}", map_nx, map_ny);

	//===========
	// VISIBILITY
	//===========

	// Visibility from NORTH
	let mut max_tree = 0;

	for x in 0..map_nx {

		// Trees on the edges are always visible
		map_tree[x][0].north = true;
		map_tree[x][0].news += 1;
		max_tree = map_tree[x][0].height;

		for y in 1..map_ny {
			if map_tree[x][y].height <= max_tree { continue;}
			map_tree[x][y].north = true;
			map_tree[x][y].news += 1;
			max_tree = map_tree[x][y].height;
		}
	}

	// Visibility from SOUTH
	let mut max_tree = 0;

	for x in 0..map_nx {

		// Trees on the edges are always visible
		map_tree[x][map_ny-1].south = true;
		map_tree[x][map_ny-1].news += 1;
		max_tree = map_tree[x][map_ny-1].height;

		for y in (0..map_ny-1).rev() {
			//print!("*{:?} ", map_tree[x][y].height);
			if map_tree[x][y].height <= max_tree { continue;}
			map_tree[x][y].south = true;
			map_tree[x][y].news += 1;
			max_tree = map_tree[x][y].height;
		}
		//println!();
	}

	// Visibility from WEST
	let mut max_tree = 0;

	for y in 0..map_ny {

		// Trees on the edges are always visible
		map_tree[0][y].west = true;
		map_tree[0][y].news += 1;
		max_tree = map_tree[0][y].height;

		for x in 1..map_nx {
			//print!("*{:?} ", map_tree[x][y].height);
			if map_tree[x][y].height <= max_tree { continue;}
			map_tree[x][y].west = true;
			map_tree[x][y].news += 1;
			max_tree = map_tree[x][y].height;
		}
		//println!();
	}

	// Visibility from EAST
	let mut max_tree = 0;

	for y in 0..map_ny {

		// Trees on the edges are always visible
		map_tree[map_nx-1][y].east = true;
		map_tree[map_nx-1][y].news += 1;
		max_tree = map_tree[map_nx-1][y].height;

		for x in (0..map_nx-1).rev() {
			//print!("*{:?} ", map_tree[x][y].height);
			if map_tree[x][y].height <= max_tree { continue;}
			map_tree[x][y].east = true;
			map_tree[x][y].news += 1;
			max_tree = map_tree[x][y].height;
		}
		//println!();
	}


	//=============
	// SCENIC SCORE
	//=============

	let mut highest_score: u32 = 0;

	for y in 0..map_ny {

		for x in 0..map_nx {
			let tree_height: u8 = map_tree[x][y].height;
			let mut score: u32  = 1;


			// Look to the NORTH
			let mut distance: u32         = 1;
			let mut viewing_distance: u32 = 1;
			let edge: usize               = 0;

			for dy in (0..y).rev() {
				let altitude: u8 = map_tree[x][dy].height;

				// if x == X && y == Y {
				// 	print!("th:{} a:{}\t", tree_height, altitude);
				// }

				if dy <= edge {
					viewing_distance = distance;
					break;
				}
				if altitude >= tree_height {
					viewing_distance = distance;
					break;
				}

				distance += 1;
			}

			if x == X && y == Y {
				println!("{} ({}, {})\t= vd:{}",
					'N', x, y, viewing_distance
				);
			}
			score *= viewing_distance;

			// Look to the SOUTH
			let mut distance: u32         = 1;
			let mut viewing_distance: u32 = 1;
			let edge: usize               = map_ny-1;

			for dy in y+1..map_ny {
				let altitude: u8 = map_tree[x][dy].height;
				
				// if x == X && y == Y {
				// 	print!("th:{} a:{}\t", tree_height, altitude);
				// }

				if dy >= edge {
					viewing_distance = distance;
					break;
				}
				if altitude >= tree_height {
					viewing_distance = distance;
					break;
				}

				distance += 1;
			}
			if x == X && y == Y {
				println!("\n{} ({}, {})\t= vd:{}",
					'S', x, y, viewing_distance
				);
			}
			score *= viewing_distance;

			// Look to the WEST
			let mut distance: u32         = 1;
			let mut viewing_distance: u32 = 1;
			let edge: usize               = 0;

			for dx in (0..x).rev() {
				let altitude: u8 = map_tree[dx][y].height;

				// if x == X && y == Y {
				// 	print!("th:{} a:{}\t", tree_height, altitude);
				// }

				if dx <= edge {
					viewing_distance = distance;
					break;
				}
				if altitude >= tree_height {
					viewing_distance = distance;
					break;
				}

				distance += 1;
			}

			if x == X && y == Y {
				println!("\n{} ({}, {})\t= vd:{}",
					'W', x, y, viewing_distance
				);
			}
			score *= viewing_distance;

			// Look to the EAST
			let mut distance: u32         = 1;
			let mut viewing_distance: u32 = 1;
			let edge: usize               = map_nx-1;

			for dx in x+1..map_nx {
				let altitude: u8 = map_tree[dx][y].height;
				
				// if x == X && y == Y {
				// 	print!("dx:{} th:{} a:{}\t", dx, tree_height, altitude);
				// }

				if dx >= edge {
					viewing_distance = distance;
					break;
				}
				if altitude >= tree_height {
					viewing_distance = distance;
					break;
				}

				distance += 1;
			}
			if x == X && y == Y {
				println!("\n{} ({}, {})\t= vd:{}",
					'E', x, y, viewing_distance
				);
			}
			score *= viewing_distance;

			if x == X && y == Y {
				println!("{} ({}, {})\t= {}", '+', x, y, score);
				println!();
			}

			if score > highest_score {
				highest_score = score;
			}

			map_tree[x][y].scenic = score;
		}
		//println!();
	}

	//println!("{:?}", map_tree);

	//========
	// DISPLAY
	//========

	println!();
	println!("NORTH");
	for y in 0..map_ny {
		for x in 0..map_nx {
			let c = match map_tree[x][y].north {
				true  => 'N',
				false => '.',
			};
			print!(" {}", c);
		}
		println!();
	}

	println!();
	println!("SOUTH");
	for y in 0..map_ny {
		for x in 0..map_nx {
			let c = match map_tree[x][y].south {
				true =>  'S',
				false => '.',
			};
			print!(" {}", c);
		}
		println!();
	}

	println!();
	println!("WEST");
	for y in 0..map_ny {
		for x in 0..map_nx {
			let c = match map_tree[x][y].west {
				true =>  'W',
				false => '.',
			};
			print!(" {}", c);
		}
		println!();
	}

	println!();
	println!("EAST");
	for y in 0..map_ny {
		for x in 0..map_nx {
			let c = match map_tree[x][y].east {
				true =>  'E',
				false => '.',
			};
			print!(" {}", c);
		}
		println!();
	}

	println!();
	println!("TOTAL");
	let mut visible: u32 = 0;

	for y in 0..map_ny {
		for x in 0..map_nx {
			let c = match map_tree[x][y].news {
				0 => ' ',
				_ => '#',
			};
			//print!(" {}", map_tree[x][y].news);
			print!(" {}", c);
			if c == '#' { visible += 1;}
		}
		println!();
	}
	println!();
	println!("SCENIC");

	for y in 0..map_ny {
		for x in 0..map_nx {
			let d = map_tree[x][y].scenic;
			print!(" {d:>3}");
		}
		println!();
	}

	println!();
	println!("Visible trees = {}", visible);

	println!();
	println!("Highest scenic score = {}", highest_score);


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

