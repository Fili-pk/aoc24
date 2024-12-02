use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let lines: Vec<Vec<i32>> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.split(' ').map(|x| x.parse().unwrap()).collect()
		})
		.collect();

	let mut scr = 0;
	for x in lines.iter() {
		let mut inc = None;
		let mut good = true;
		for x in x.windows(2) {
			if (x[0] - x[1]).abs() > 3 || x[0] == x[1] {
				good = false;
				break;
			}

			let Some(inc) = inc else {
				if x[0] < x[1] {
					inc = Some(true)
				} else if x[0] > x[1] {
					inc = Some(false)
				}

				continue;
			};

			if (x[0] > x[1]) == inc {
				good = false;
				break;
			}
		}

		if good {
			scr += 1;
		}
	}

	println!("{scr}");
}
