use itertools::Itertools;
use std::{
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
			x.parse::<u64>().unwrap()
		})
		.collect();

	let mut sum = 0;
	for mut x in lines {
		for i in 0..2000 {
			x ^= (x << 6);
			x %= 16777216;
			x ^= (x >> 5);
			x %= 16777216;
			x ^= (x << 11);
			x %= 16777216;
		}
		sum += x;
	}

	println!("{sum}")
}
