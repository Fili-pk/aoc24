use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
	usize,
};
use utils::*;

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut lines = file.lines().map(|x| {
		let x = x;
		x.unwrap()
	});
	let mut a = lines
		.next()
		.unwrap()
		.split_once(": ")
		.unwrap()
		.1
		.parse::<u64>()
		.unwrap();
	let mut b = lines
		.next()
		.unwrap()
		.split_once(": ")
		.unwrap()
		.1
		.parse::<u64>()
		.unwrap();
	let mut c = lines
		.next()
		.unwrap()
		.split_once(": ")
		.unwrap()
		.1
		.parse::<u64>()
		.unwrap();
	lines.next();

	let ins = lines
		.next()
		.unwrap()
		.split_once(": ")
		.unwrap()
		.1
		.split(',')
		.map(|x| x.parse::<u64>().unwrap())
		.collect::<Vec<_>>();

	let mut ptr = 0;
	tryy(|| 'a: loop {
		let opr = || ins[ptr + 1];
		let cmb = || match opr() {
			0 => 0,
			1 => 1,
			2 => 2,
			3 => 3,
			4 => a,
			5 => b,
			6 => c,
			_ => panic!(),
		};
		match ins[ptr] {
			0 => a /= 1 << cmb(),
			1 => b ^= opr(),
			2 => b = cmb() % 8,
			3 if a != 0 => {
				ptr = opr() as usize;
				continue 'a;
			}
			3 => {}
			4 => {
				opr();
				b ^= c
			}
			5 => print!("{},", cmb() % 8),
			6 => b = a / (1 << cmb()),
			7 => c = a / (1 << cmb()),
			_ => panic!(),
		}

		ptr += 2;
	});
	println!();
}
