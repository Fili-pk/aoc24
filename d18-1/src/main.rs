use itertools::Itertools;
use std::{
	collections::VecDeque,
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			let x = x.split_once(",").unwrap();
			(x.1.parse::<usize>().unwrap(), x.0.parse::<usize>().unwrap())
		})
		.collect();
	const SIZE: usize = 71;
	let mut ram = [[true; SIZE]; SIZE];
	for a in lines.iter().take(1024) {
		ram[a.0][a.1] = false;
	}

	let mut visited = [[false; SIZE]; SIZE];
	let mut que = VecDeque::new();
	que.push_back((0, 0));
	let mut depth = 0;
	'q: while !que.is_empty() {
		let len = que.len();
		for _ in 0..len {
			let c = que.pop_front();
			let Some(c) = c else {
				continue;
			};

			let Some(v) = visited.geti_mut(c.0).and_then(|x| x.geti_mut(c.1)) else {
				continue;
			};

			if *v {
				continue;
			}

			if c.0 as usize == SIZE - 1 && c.1 as usize == SIZE - 1 {
				break 'q;
			}

			*v = true;

			if ram[c.0 as usize][c.1 as usize] {
				que.push_back((c.0 + 1, c.1));
				que.push_back((c.0, c.1 + 1));
				que.push_back((c.0 - 1, c.1));
				que.push_back((c.0, c.1 - 1));
			}
		}
		depth += 1;
	}

	println!("{depth}")
}
