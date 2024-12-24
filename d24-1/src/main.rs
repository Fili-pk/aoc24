use itertools::Itertools;
use std::{
	collections::HashMap,
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
			x
		})
		.collect();

	let mut h = HashMap::new();
	for x in lines.iter().take_while(|x| !x.is_empty()) {
		let x = x.split_once(": ").unwrap();
		h.insert(x.0.to_owned(), x.1 == "1");
	}

	let mut d = vec![];
	for x in lines.iter().filter(|x| x.contains("->")) {
		let x = x.split_once(" ").unwrap();
		let x2 = x.1.split_once(" ").unwrap();
		let x3 = x2.1.split_once(" -> ").unwrap();
		d.push((
			x.0.to_owned(),
			x2.0.to_owned(),
			x3.0.to_owned(),
			x3.1.to_owned(),
		));
	}

	while !d.is_empty() {
		let mut i = 0;
		while i < d.len() {
			let x = &d[i];
			let Some(_) = h.get(&x.0) else {
				i += 1;
				continue;
			};
			let Some(_) = h.get(&x.2) else {
				i += 1;
				continue;
			};

			let x = d.swap_remove(i);
			let x1 = *h.get(&x.0).unwrap();
			let x2 = *h.get(&x.2).unwrap();
			let v = match x.1.as_str() {
				"AND" => x1 && x2,
				"XOR" => x1 != x2,
				"OR" => x1 || x2,
				x => panic!("{}", x),
			};

			h.remove(&x.3);
			h.insert(x.3.clone(), v);
		}
	}

	let v = h
		.into_iter()
		.filter(|x| x.0.starts_with("z"))
		.sorted_by_key(|x| x.0.clone())
		.collect::<Vec<_>>();

	let mut val: u64 = 0;
	for x in v.iter().rev() {
		val <<= 1;
		val |= x.1 as u8 as u64;
	}

	println!("{val}");
}
