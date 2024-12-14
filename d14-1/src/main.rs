use itertools::Itertools;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};
use utils::*;

struct R {
	x: i32,
	y: i32,
	v_x: i32,
	v_y: i32,
}

fn main() {
	let file = BufReader::new(File::open("input").unwrap());
	let mut lines: Vec<_> = file
		.lines()
		.map(|x| {
			let x = x.unwrap();
			let (a, b) = x.split_once(" ").unwrap();
			let (a1, a2) = a.split_once(",").unwrap();
			let (b1, b2) = b.split_once(",").unwrap();
			R {
				x: a1.trim_start_matches("p=").parse().unwrap(),
				y: a2.parse().unwrap(),
				v_x: b1.trim_start_matches("v=").parse().unwrap(),
				v_y: b2.parse().unwrap(),
			}
		})
		.collect();

	for x in lines.iter_mut() {
		x.x += x.v_x * 100 + 10000 * 101;
		x.y += x.v_y * 100 + 10000 * 103;
		x.x %= 101;
		x.y %= 103;
	}

	let mut a = 0;
	let mut b = 0;
	let mut c = 0;
	let mut d = 0;

	for x in lines.iter_mut() {
		let w = (0..50).contains(&x.x);
		let nw = (51..101).contains(&x.x);
		let q = (0..51).contains(&x.y);
		let nq = (52..103).contains(&x.y);

		if w && q {
			a += 1;
			continue;
		}

		if nw && q {
			b += 1;
			continue;
		}

		if w && nq {
			c += 1;
			continue;
		}

		if nw && nq {
			d += 1;
			continue;
		}
	}

	println!("{}", a * b * c * d)
}
