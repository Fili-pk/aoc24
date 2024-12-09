use itertools::Itertools;
use std::{
	fmt::DebugStruct,
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.chars()
				.map(String::from)
				.map(|x| x.parse::<u8>().unwrap())
				.enumerate()
				.map(|x| (x.1, x.0 / 2))
				.collect()
		})
		.next()
		.unwrap();

	if lines.len() % 2 == 0 {
		lines.pop();
	}

	let mut idx = (lines.len() - 1) as i32;
	while idx >= 0 {
		let d = lines[idx as usize].0;
		if d == 0 {
			idx -= 2;
			continue;
		}

		let Some(i) = lines
			.iter()
			.enumerate()
			.take(idx as usize)
			.filter(|x| x.0 % 2 == 1)
			.filter(|x| x.1 .0 >= d && x.1 .0 != 0)
			.next()
			.map(|x| x.0)
		else {
			idx -= 2;
			continue;
		};

		lines[i].0 -= lines[idx as usize].0;
		lines[idx as usize - 1].0 += lines[idx as usize].0;
		lines.insert(i, lines[idx as usize]);
		lines.insert(i, (0, 0));

		lines[idx as usize + 2] = (0, 0);
	}

	let mut dec: Vec<Option<u64>> = Vec::new();
	for (i, x) in lines.iter().enumerate() {
		for j in 0..x.0 {
			dec.push(if i % 2 == 0 { Some(x.1 as u64) } else { None });
		}
	}

	let sum: u64 = dec
		.into_iter()
		.enumerate()
		.map(|x| (x.0 as u64) * x.1.unwrap_or(0))
		.sum();

	println!("{sum:?}")
}
