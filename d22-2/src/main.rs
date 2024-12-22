use itertools::Itertools;
use std::{
	collections::{HashMap, HashSet, VecDeque},
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
	let mut h = HashMap::new();
	'q: for mut x in lines {
		let y = x.clone();
		let mut x_max = 0;
		for i in 0..10 {
			x ^= (x << 6);
			x %= 16777216;
			x ^= (x >> 5);
			x %= 16777216;
			x ^= (x << 11);
			x %= 16777216;
			if x % 10 > x_max {
				x_max = x % 10;
			}
		}

		let mut d = VecDeque::new();
		d.push_back(0);
		let mut lastt = (y % 10) as i64;
		let mut x = y;
		for i in 0..3 {
			x ^= (x << 6);
			x %= 16777216;
			x ^= (x >> 5);
			x %= 16777216;
			x ^= (x << 11);
			x %= 16777216;

			let s = (x % 10) as i64;
			d.push_back(s - lastt);
			//println!("a{} {s} {x}", s - lastt);
			lastt = s;
		}

		let mut h2 = HashSet::new();
		for i in 0..1996 {
			d.pop_front();
			x ^= (x << 6);
			x %= 16777216;
			x ^= (x >> 5);
			x %= 16777216;
			x ^= (x << 11);
			x %= 16777216;
			let s = (x % 10) as i64;
			d.push_back(s - lastt);
			//println!("{} {s} {x}", s - lastt);
			if !h2.contains(&d) {
				h.entry(d.clone()).and_modify(|x| *x += s).or_insert(s);
			}
			h2.insert(d.clone());
			lastt = s;
		}
	}

	let www = h.iter().max_by_key(|x| *x.1).unwrap();
	println!("{:?}", www);
}
