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
	let mut qqq = 0;
	for d in [0u64, 1, 2, 3, 4, 5, 6, 7, 8].c_product::<16>() {
		let mut i = 0;
		for (j, x) in d.iter().enumerate() {
			i += 8_u64.pow(15 - j as u32) * *x
		}
		let mut w = vec![];
		let mut a = i;
		let mut b = 0;
		let mut c = 0;

		loop {
			b = a % 8;
			b ^= 1;
			c = a / 2_u64.pow(b as u32);
			b ^= c;
			a /= 8;
			b ^= 6;
			w.push(b % 8);
			if a == 0 {
				break;
			}
		}

		if w.len() == 16 && w.ends_with(&[0, 3, 1, 6, 5, 5, 3, 0]) {
			qqq = i;
			break;
		}
	}

	for i in qqq.. {
		let mut w = vec![];
		let mut a = i;
		let mut b = 0;
		let mut c = 0;

		loop {
			b = a % 8;
			b ^= 1;
			c = a / 2_u64.pow(b as u32);
			b ^= c;
			a /= 8;
			b ^= 6;
			w.push(b % 8);
			if a == 0 {
				break;
			}
		}

		if w == [2, 4, 1, 1, 7, 5, 4, 0, 0, 3, 1, 6, 5, 5, 3, 0] {
			println!("{i}");
			break;
		}
	}
}
