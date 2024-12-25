use itertools::Itertools;
use std::{
	collections::HashSet,
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
			x.chars().collect::<Vec<_>>()
		})
		.collect();

	let mut v = HashSet::new();
	for x in lines.split(|x| x.is_empty()) {
		let mut l = vec![];
		if x[0][0] == '#' {
			for i in 0..x[0].len() {
				l.push(0);
				for (j, x) in x.iter().enumerate() {
					if x[i] == '.' {
						l[i] = j as i32;
						break;
					}
				}
			}
		} else {
			for i in 0..x[0].len() {
				l.push(0);
				for (j, x) in x.iter().rev().enumerate() {
					if x[i] == '.' {
						l[i] = -(j as i32);
						break;
					}
				}
			}
		}
		println!("{l:?}");
		v.insert(l);
	}

	let mut scr = 0;
	println!("{v:#?}");
	'w: for x in v.iter().combinations(2) {
		let x1 = x[0];
		let x2 = x[1];
		if x1[0].signum() == x2[0].signum() {
			continue;
		}

		for (a, b) in x1.iter().zip(x2.iter()) {
			if a.abs() + b.abs() > 7 {
				continue 'w;
			}
		}

		println!("{:?} {:?}", x1, x2);
		scr += 1;
	}

	println!("{scr}")
}
