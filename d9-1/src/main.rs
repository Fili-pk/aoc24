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
	let lines: Vec<u8> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			x.chars()
				.map(String::from)
				.map(|x| x.parse::<u8>())
				.map(|x| x.unwrap())
				.collect()
		})
		.next()
		.unwrap();

	let mut i = 1;
	while i < lines.len() {
		let d = lines[i];
		lines
			.iter()
			.enumerate()
			.skip(i)
			.filter(|x| x.0 % 2 == 0)
			.filter(|x| *x.1 >= d);

		i += 1;
	}

	let mut dec: Vec<Option<u64>> = Vec::new();
	for (i, x) in lines.iter().enumerate() {
		let i = i as u64;
		for j in 0..*x {
			dec.push(if i % 2 == 0 { Some(i / 2) } else { None });
		}
	}

	let mut i = 0;
	while i < dec.len() {
		if dec[i].is_none() {
			dec.swap_remove(i);
			while let Some(None) = dec.last() {
				dec.pop();
			}
			continue;
		}

		i += 1;
	}

	let sum: u64 = dec
		.into_iter()
		.enumerate()
		.map(|x| (x.0 as u64) * x.1.unwrap())
		.sum();

	println!("{sum}")
}
